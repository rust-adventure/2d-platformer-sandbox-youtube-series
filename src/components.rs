use crate::actions::PlatformerAction;
use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::*,
    utils::ldtk_pixel_coords_to_translation_pivoted,
};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use std::collections::HashSet;

// use heron::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass_properties: ColliderMassProperties,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(
        entity_instance: EntityInstance,
    ) -> ColliderBundle {
        let rotation_constraints =
            LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(6., 14.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            "Mob" => ColliderBundle {
                collider: Collider::cuboid(5., 5.),
                rigid_body:
                    RigidBody::KinematicVelocityBased,
                rotation_constraints,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                friction: Friction {
                    coefficient: 0.5,
                    combine_rule:
                        CoefficientCombineRule::Min,
                },
                restitution: Restitution {
                    coefficient: 0.7,
                    combine_rule:
                        CoefficientCombineRule::Min,
                },
                mass_properties:
                    ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 2 {
            ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rotation_constraints:
                    LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            }
        } else {
            ColliderBundle::default()
        }
    }
}

#[derive(Bundle)]
pub struct PlayerInput {
    #[bundle]
    input: InputManagerBundle<PlatformerAction>,
}
impl Default for PlayerInput {
    fn default() -> Self {
        use PlatformerAction::*;

        let mut input_map = InputMap::default();

        // basic movement
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(
            SingleGamepadAxis::symmetric(
                GamepadAxisType::LeftStickX,
                0.1,
            ),
            Horizontal,
        );

        input_map.insert(KeyCode::D, Right);
        input_map
            .insert(GamepadButtonType::DPadRight, Right);

        // Jump
        input_map
            .insert(KeyCode::Space, PlatformerAction::Jump);
        input_map.insert(
            GamepadButtonType::South,
            PlatformerAction::Jump,
        );

        input_map
            .insert(KeyCode::E, PlatformerAction::Dash);
        input_map.insert(
            GamepadButtonType::RightTrigger2,
            PlatformerAction::Dash,
        );

        input_map.insert(
            KeyCode::Return,
            PlatformerAction::Pause,
        );
        input_map.insert(
            GamepadButtonType::Start,
            PlatformerAction::Pause,
        );

        input_map
            .insert(KeyCode::I, PlatformerAction::Menus);
        input_map.insert(
            GamepadButtonType::Select,
            PlatformerAction::Menus,
        );
        input_map.set_gamepad(Gamepad(0));
        Self {
            input: InputManagerBundle::<PlatformerAction> {
                input_map,
                ..Default::default()
            },
        }
    }
}

#[derive(
    Clone, Component, Debug, Eq, Default, PartialEq,
)]
pub struct Items(Vec<String>);

impl From<EntityInstance> for Items {
    fn from(entity_instance: EntityInstance) -> Self {
        let mut items: Vec<String> = vec![];

        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"items")
        {
            // convert &String to String which returns
            // vec![String::from("Knife"),
            // String::from("Boot")]
            items = match &field_instance.value {
                FieldValue::Enums(v) => v
                    .iter()
                    .flatten()
                    .map(|s| s.into())
                    .collect::<Vec<String>>(),
                _ => vec![],
            };
        }

        Self(items)
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Component,
)]
pub struct Player;

#[derive(
    Clone, Eq, PartialEq, Debug, Default, Component,
)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle(
        "zombie_tilesheet.png",
        80.,
        113.,
        9,
        3,
        0.,
        0
    )]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub gravity_scale: GravityScale,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,

    // Build Items Component manually by using `impl
    // From<EntityInstance>
    #[from_entity_instance]
    items: Items,

    // The whole EntityInstance can be stored directly as
    // an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[bundle]
    pub input: PlayerInput,
}

#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Component,
)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Component,
)]
pub struct Climbable;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub climbable: Climbable,
}

#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Default, Component,
)]
pub struct Enemy;

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(
            ldtk_pixel_coords_to_translation_pivoted(
                entity_instance.px,
                layer_instance.c_hei
                    * layer_instance.grid_size,
                IVec2::new(
                    entity_instance.width,
                    entity_instance.height,
                ),
                entity_instance.pivot,
            ),
        );

        let ldtk_patrol = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"patrol")
            .unwrap();
        if let FieldValue::Points(ldtk_points) =
            &ldtk_patrol.value
        {
            for ldtk_point in ldtk_points {
                if let Some(ldtk_point) = ldtk_point {
                    // The +1 is necessary here due to the
                    // pivot of the entities in the sample
                    // file.
                    // The patrols set up in the file look
                    // flat and grounded,
                    // but technically they're not if you
                    // consider the pivot,
                    // which is at the bottom-center for the
                    // skulls.
                    let pixel_coords = (ldtk_point
                        .as_vec2()
                        + Vec2::new(0.5, 1.))
                        * Vec2::splat(
                            layer_instance.grid_size as f32,
                        );

                    points.push(ldtk_pixel_coords_to_translation_pivoted(
                        pixel_coords.as_ivec2(),
                        layer_instance.c_hei * layer_instance.grid_size,
                        IVec2::new(entity_instance.width, entity_instance.height),
                        entity_instance.pivot,
                    ));
                }
            }
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct MobBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    #[ldtk_entity]
    pub patrol: Patrol,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}
