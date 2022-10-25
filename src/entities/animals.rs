use crate::{prelude::*, utils::entity_destruct};

const ANIMAL_SPEED: f32 = 40.0;
const ANIMAL_ACC: f32 = 200.0;
const STROLL_TIME: f32 = 1.0;
const REACT_DISTANCE: f32 = 60.0;

pub struct AnimalsPlugin;

impl Animal
{
    pub fn set_random_stroll(&mut self) {
        let mut rng = thread_rng();
        let rand_vec = Vec3::new(rng.gen::<f32>()-0.5, rng.gen::<f32>()-0.5, 0.0).normalize();
        self.stroll_direction = rand_vec;
    }
}

impl Default for Animal
{
    fn default() -> Self {
        let mut animal = Animal {
            stroll_timer: Timer::from_seconds(STROLL_TIME, true),
            stroll_direction: Vec3::NAN
        };

        animal.set_random_stroll();

        animal
    }
}

impl Plugin for AnimalsPlugin
{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(animal_behaviour)
        .with_system(entities::mutual_repulsion::<Animal>));
    }
}

fn animal_behaviour(
    mut animal_query: Query<(&Transform, &mut Rigidbody, &mut Animal), With<Animal>>,
    hostile_query: Query<&Transform, Or<(With<Zombie>, With<Player>)>>,
    static_objs: Query<&Transform, With<StaticEntity>>,
    time: Res<Time>
) {
    let static_vec_trans: Vec<&Transform> = static_objs.iter().collect();

    'outer: for (anim_trans, mut anim_rb, mut animal) in animal_query.iter_mut() {
        for hostile_trans in hostile_query.iter() {
            let vec_away: Vec3 = anim_trans.translation - hostile_trans.translation;
            if vec_away.length() < REACT_DISTANCE {
                if !is_hindered(&static_vec_trans, &anim_trans, &hostile_trans) {
                    anim_rb.acc_clamped(vec_away.normalize(), ANIMAL_ACC, ANIMAL_SPEED, &time);

                    continue 'outer;
                }
            }
        }

        if !animal.stroll_timer.tick(time.delta()).just_finished() {
            anim_rb.acc_clamped(animal.stroll_direction, ANIMAL_ACC, ANIMAL_SPEED/4.0, &time);
        } else {
            animal.set_random_stroll();
        }
    }
}

pub fn spawn_animal(
    commands: &mut Commands,
    spawn_pos: Vec3,
    game_assets: &Res<GameAssets>
) {
    let mut rng = rand::thread_rng();

    (*commands)
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: 21 + rng.gen_range(0..2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(TransformBundle{
            local: Transform::from_translation(spawn_pos),
            ..Default::default()
        })
        .insert(Rigidbody{
            vx: 0.0,
            vy: 0.0,
            friction: true
        })
        .insert(Animal {
            ..Default::default()
        })
        .insert(BoxCollider {
            size: Vec2::new(10.0, 10.0)
        })
        .insert(Attackable(TargetPriority::High))
        .insert(EntityHealth{val: 20.0, func_destruct: entity_destruct});
}