use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default(), GlobalTransform::default()));
}

//For now, the camera is just a basic 2D camera centered at the origin.
//For later: Possibly adda a 3D camera and controls like panning and zooming in!