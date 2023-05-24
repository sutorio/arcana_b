//! WIP: The core camera.
//!
//! This is moving towards an emulation of a Godot camera attached to a spring arm.
//! So the focus of the camera "arm" entity should be placed at the centre of the target
//! (*ie* the currently controlled character). It's just a SpatialBundle. This provides
//! the Transform component. Then attached as a child is the camera entity. It being a
//! child means that the transform applied to it will be relative (move it up Y,
//! move it back X, look at the focus). Following Godot, what the camera also requires
//! is a collider: this can then be used to adjust the "arm". Also need to look at some
//! spring physics.
use bevy::prelude::*;
use std::f32::consts::PI;

/// Initialises the blend camera and its associated functionality
pub struct OrbitCameraSetupPlugin {
    pub initial_position: Vec3,
    pub initial_framing: Vec2,
}

impl Plugin for OrbitCameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(OrbitCameraParams {
                distance: -5.0,
                framing: self.initial_framing,
                pitch: 10.0,
                tracking_position: self.initial_position,
                yaw: 0.0,
            })
            .add_systems(Startup, spawn_orbit_camera)
            .add_systems(Update, log_camera_updates)
            .add_systems(Update, (compute_camera_blend, apply_blend_to_params, apply_params_to_camera).chain());
    }
}

#[derive(Component)]
pub struct OrbitCamera;

#[derive(Resource, Debug, Default)]
pub struct OrbitCameraParams {
	/// Offset between the camera and the player
	pub distance: f32,
	/// 2D screen position of the player
	pub framing: Vec2,
	/// Vertical camera tilt
	pub pitch: f32,
	/// 3D world position of the player
    pub tracking_position: Vec3,
	/// Horizontal camera pan
	pub yaw: f32,
}

impl OrbitCameraParams {
    /// Inverse-compute the camera parameters from the current camera and the target position.
    fn derive(
        camera_projection: &PerspectiveProjection,
        camera_transform: &Transform,
        tracking_position: Vec3,
    ) -> Self {
        let camera_position = camera_transform.translation;
        let camera_rotation = camera_transform.rotation;

        // compute pitch and yaw values from the camera rotation
        let (pitch, yaw, _) = camera_rotation.to_euler(EulerRot::XYZ);

        // compute distance from the camera position and the tracking position
        let tracking_offset = tracking_position - camera_position;
        let fwd = camera_rotation * Vec3::Z;
        let distance = tracking_offset.dot(fwd);

        // info!("COMPUTING FRAMING:");
        // compute framing from the camera position and the tracking position
        let camera_offset = camera_position - tracking_position;
        // info!("Offset: {:?}", camera_offset);
        let parallax = camera_rotation.inverse() * camera_offset;
        // info!("Parallax: {:?}", parallax);
        let tan_fov_y = (0.5 * (std::f32::consts::PI / 180.0) * camera_projection.fov).tan();
        // info!("tan(fov_y): {:?}", tan_fov_y);
        let tan_fov_x = tan_fov_y * camera_projection.aspect_ratio;
        // info!("tan(fov_x): {:?}", tan_fov_x);
        let screen_to_world = distance * Vec2::new(tan_fov_x, tan_fov_y);
        // info!("Screen -> World: {:?}", screen_to_world);
        let framing = Vec2::new(
            (-parallax.x / screen_to_world.x).clamp(-1.0, 1.0),
            (-parallax.y / screen_to_world.y).clamp(-1.0, 1.0),
        );
        // info!("Framing: {:?}", framing);
        // info!(":END COMPUTING FRAMING");

        Self {
            distance,
            framing,
            pitch,
            tracking_position,
            yaw,
        }
    }
}

fn spawn_orbit_camera(mut commands: Commands, params: Res<OrbitCameraParams>) {
    commands.spawn((
        OrbitCamera,
        Camera3dBundle {
            transform: Transform::from_translation(params.tracking_position).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn log_camera_updates(
    query: Query<&Transform, (With<OrbitCamera>, Changed<Transform>)>,
    params: Res<OrbitCameraParams>
) {
    for transform in query.iter() {
        info!("Camera transform: {:?}", transform);
    }

    if params.is_changed() {
        info!("Camera params: {:?}", params);
    }
}

/// The CameraBlend is local resource used internally to track the difference between previous and
/// current "parameter spaces". It holds the same values as the CameraParams component, less the
/// tracking_position (which comes from the world). By saving the values separately, we
/// can use them to compute the camera transform in the next frame.
#[derive(Default)]
struct CameraBlend {
    framing: Vec2,
    distance: f32,
    pitch: f32,
    yaw: f32,
}


fn compute_camera_blend(
    camera: Query<(&Projection, &Transform), With<OrbitCamera>>,
    params: Res<OrbitCameraParams>,
    mut blend: Local<CameraBlend>
) {
    if params.is_changed() || params.is_added() {
        for (projection, transform) in camera.iter() {
            if let Projection::Perspective(camera_projection) = projection {
                let old_params = OrbitCameraParams::derive(&camera_projection, transform, params.tracking_position);

                blend.distance = old_params.distance - params.distance;
                blend.framing = old_params.framing - params.framing;
                blend.pitch = old_params.pitch - params.pitch;
                // NOTE: yaw is a special case because it wraps around. If this isn't done, will get wierd jumps.
                blend.yaw = match old_params.yaw - params.yaw {
                    yaw if yaw > 180.0 => yaw - 360.0,
                    yaw if yaw < -180.0 => yaw + 360.0,
                    yaw => yaw
                };
            }
        }
    }
}

fn apply_blend_to_params(
    mut params: ResMut<OrbitCameraParams>,
    blend: Local<CameraBlend>,
    time: Res<Time>,
) {
        let multi = 1.0 - time.delta_seconds().clamp(0.0, 1.0);

        params.distance += blend.distance * multi;
        params.framing += blend.framing * multi;
        params.pitch += blend.pitch * multi;
        params.yaw += blend.yaw * multi;
}

fn apply_params_to_camera(
    mut camera: Query<(&Projection, &mut Transform), With<OrbitCamera>>,
    params: Res<OrbitCameraParams>,
) {
    if params.is_changed() {
        for (projection, mut transform) in camera.iter_mut() {
            if let Projection::Perspective(camera_projection) = projection {
                // Compute "local" offset relative to our view rotation
                // REVIEW: why `tan`, what does `tan` do here?
                let tan_fov_y = (0.5 * (PI / 180.0) * camera_projection.fov).tan();
                let tan_fov_x = tan_fov_y * camera_projection.aspect_ratio;
                let local_offset = Vec3::new(
                    params.distance * tan_fov_x * params.framing.x,
                    params.distance * tan_fov_y * params.framing.y,
                    params.distance
                );

                // Compute rotation, and thence translation. Just for reference:
                //
                // - pitch is direction of wing (y),
                // - yaw is direction of ground (z),
                // - roll is forward motion (x)
                //
                // REFERENCE: https://www.mecharithm.com/explicit-representations-orientation-robotics-roll-pitch-yaw-angles/
                let rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    params.pitch,
                    params.yaw,
                    0.0,
                );
                let translation = params.tracking_position - rotation * local_offset;

                transform.translation = translation;
                transform.rotation = rotation;
            }
        }
    }
}
