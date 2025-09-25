use bevy::prelude::*;

pub enum Axis{
    X,
    Y,
    Z,
}

#[derive(Component)]
pub struct UIDimentionBlock{
    pub dimention: u8,
    pub axis: Axis,
    pub value: u8,
}

pub fn create_dimention_blocks(mut commands: Commands){
    create_dimention_block(&mut commands, 1);
    create_dimention_block(&mut commands, 2);
    create_dimention_block(&mut commands, 3);
    create_dimention_block(&mut commands, 4);
    create_dimention_block(&mut commands, 5);
}

fn create_dimention_block(commands: &mut Commands, dimention: u8){
    let block = (
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(dimention as f32 * 35.0),
            left: Val::Px(15.0),
            width: Val::Px(500.),
            height: Val::Px(30.),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        children![
            Text::new(dimention.to_string()),
            TextFont {
                font_size: 30.0,
                ..Default::default()
            },
        ],
    );

    commands.spawn(block);
}