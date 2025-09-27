use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

pub enum Axis{
    None,
    X,
    Y,
    Z,
}

#[derive(Component)]
pub struct UIDimentionBlock{
    pub dimention: u8,
    pub axis: Axis,
    pub value: f32,
}

impl UIDimentionBlock {
    const fn new(dimention: u8) -> Self {
        Self {
            dimention,
            axis: Axis::X,
            value: 0.0,
        }
    }
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
        UIDimentionBlock::new(dimention),
        ZIndex(1),
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        children![
            ( // Dimension number
                Text::new(dimention.to_string()),
                TextFont {
                    font_size: 26.0,
                    ..Default::default()}
            ),
            ( // Axis selector
                Node{
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(35.0),
                    width: Val::Px(90.),
                    height: Val::Px(30.),
                    ..default()
                },
                ZIndex(2),
                BackgroundColor(Color::srgb(0.80, 0.25, 0.25)),
                children![
                    create_axis_selector(Axis::X),
                    create_axis_selector(Axis::Y),
                    create_axis_selector(Axis::Z),
                ]
            ),
            create_value_selector(),
        ],
    );

    commands.spawn(block);
}

fn create_axis_selector(axis: Axis) -> impl Bundle {
    let color = match axis {
        Axis::None => Color::srgb(0.50, 0.50, 0.50),
        Axis::X => Color::srgb(0.80, 0.30, 0.30),
        Axis::Y => Color::srgb(0.30, 0.80, 0.30),
        Axis::Z => Color::srgb(0.30, 0.30, 0.80),
    };

    let text = match axis {
        Axis::None => "-",
        Axis::X => "X",
        Axis::Y => "Y",
        Axis::Z => "Z",
    };

    let offset = match axis {
        Axis::None => 0.0,
        Axis::X => 0.0,
        Axis::Y => 30.0,
        Axis::Z => 60.0,
    };

    return (
        Node{
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(offset),
            width: Val::Px(30.),
            height: Val::Px(30.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        ZIndex(3),
        BackgroundColor(color),
        children![
            Text::new(text),
            TextFont {
                font_size: 26.0,
                ..Default::default()}
        ],
    );
}

fn create_value_selector() -> impl Bundle {
    return (
        ( // Background
            Node{ 
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(140.0),
                width: Val::Px(300.),
                height: Val::Px(30.),
                ..default()
            },
            ZIndex(2),
            BackgroundColor(Color::srgb(0.60, 0.60, 0.60)),
            Interaction::None,
            RelativeCursorPosition::default(),
            children![
                ( // Foreground slider
                    Node{
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        width: Val::Px(300.),
                        height: Val::Px(30.),
                        ..default()
                    },
                    ZIndex(3),
                    BackgroundColor(Color::srgb(0.60, 1.00, 0.60)),
                    Interaction::None,
                    RelativeCursorPosition::default(),
                ),
                (
                Text::new(0.0.to_string()),
                TextFont {
                    font_size: 26.0,
                    ..Default::default()},
                ZIndex(4),
                )
            ],
        ),
    );
}

// Update dimension value and text
pub fn update_value_selector(
    query: Query<(Entity, &UIDimentionBlock)>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
    mut node_query: Query<&mut Node>,
){
    for (entity, dimension_block) in query.iter() {

        let value_selector = children_query.get(entity).unwrap().get(2).unwrap();

        let slider = children_query.get(*value_selector).unwrap().get(0).unwrap();
        let text = children_query.get(*value_selector).unwrap().get(1).unwrap();

        if let Ok(mut text_elem) = text_query.get_mut(*text) {
            text_elem.0 = ((dimension_block.value * 100.0) as u8).to_string();
        }

        if let Ok(mut node_elem) = node_query.get_mut(*slider) {
            node_elem.width = Val::Px(dimension_block.value as f32 * 300.0);
        }
    }
}

// Control dimension value by dragging the slider
pub fn control_dimention_value_selector(
    query: Query<(&Interaction, &RelativeCursorPosition, &ChildOf)>,
    mut dimention_block_query: Query<&mut UIDimentionBlock>
){
    for (interaction, relative_cursor_position, child_of) in query.iter() {
        if *interaction == Interaction::Pressed {
            println!("interaction: {:?}", interaction);
            println!("relative_cursor_position: {:?}", relative_cursor_position);

            let parent = child_of.parent();

            if let Ok(mut parent_elem) = dimention_block_query.get_mut(parent) {
                parent_elem.value = relative_cursor_position.normalized.unwrap().x.clamp(0.0, 1.0);
            }
        }
    }
}