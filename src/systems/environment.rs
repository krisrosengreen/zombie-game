use crate::{prelude::*, utils::entity_destruct};

const NUM_TREES: u8 = 50;
const NUM_ANIMALS: u8 = 25;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system_set(SystemSet::on_enter(AppState::GameSetup).with_system(spawn_environment));
    }
}

fn spawn_environment(
    mut commands: Commands,
    game_assets: Res<GameAssets>
) {
    // Spawn grassy background!
    let grass_indeces = [2,3,6,7];
    let mut rng = rand::thread_rng();

    let lower: i8 = -35;
    let upper: i8 = 35;

    for x in lower..upper
    {
        for y in lower..upper {

            commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_assets.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: grass_indeces[rng.gen_range(0..4)],
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(TransformBundle{
                local: Transform::from_xyz(20.0 * (x as f32), 20.0 * (y as f32), 0.0),
                ..Default::default()
            });

        }
    }

    // Spawn trees

    for _ in 0..NUM_TREES{
        let xtile = rng.gen_range(-20..20);
        let ytile = rng.gen_range(-20..20);

        spawn_tree(&mut commands, Vec3::new((xtile as f32)*20.0, (ytile as f32)*20.0, 3.0), &game_assets);
    }

    // Spawn animals

    for _ in 0..NUM_ANIMALS {
        let xtile = rng.gen_range(-20..20);
        let ytile = rng.gen_range(-20..20);

        animals::spawn_animal(&mut commands, Vec3::new((xtile as f32)*20.0, (ytile as f32)*20.0, 3.0), &game_assets);
    }

}

fn spawn_tree(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_asset: &Res<GameAssets>
) {

    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_asset.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 8,
                custom_size: Some(Vec2 { x: 20.0, y: 20.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(StaticEntity)
        .insert(Attackable(TargetPriority::Low))
        .insert(EntityHealth{val: 200.0, func_destruct: entity_destruct})
        .insert(DropsItem{
            item: Item {
                item_type: SelectionTypes::WallBlock,
                quantity: 3
            }
        })
        .insert(BoxCollider {
            size: Vec2::new(20.0, 20.0)
        });
}