// mod hidden_object;
// mod time_management;

use crate::base::AppState;
use bevy::{pbr::resources, prelude::*, render::view::visibility};

pub struct LevelPlugin;

// TODO: refactor into separate game types
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Level), setup_level)
            .add_event::<WorkerRequest>()
            .add_systems(
                Update,
                (
                    activate_available_worker,
                    show_worker,
                    move_worker,
                    animate_worker,
                    complete_task,
                    update_home,
                )
                    .chain()
                    .run_if(in_state(AppState::Level)),
            );
    }
}

// TODO: add time comppnent
// when completing tasks
#[derive(Component, Copy, Clone)]
enum Resource {
    Food(u32),
    // Wood(u32),
    // Gold(u32),
}

// #[derive(Component)]
// struct Target<T>(T);

// TODO: add time component!
#[derive(Event, Copy, Clone)]
struct WorkerRequest {
    resources: Option<Resource>,
    position: Vec3,
}

impl WorkerRequest {
    fn new(resources: Option<Resource>, position: Vec3) -> WorkerRequest {
        WorkerRequest {
            resources,
            position,
        }
    }
}

#[derive(Component, Copy, Clone)]
struct WorkerTask {
    resources: Option<Resource>,
    position: Vec3,
}

impl From<WorkerRequest> for WorkerTask {
    fn from(item: WorkerRequest) -> WorkerTask {
        WorkerTask {
            resources: item.resources,
            position: item.position,
        }
    }
}

#[derive(Component)]
struct HomeTag;

#[derive(Component)]
enum WorkerState {
    AtHome,
    ToHome,
    ToTarget,
    AtTarget,
    Passive,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // loading_query: Query<Entity, With<LoadingTag>>,
) {
    let barrel = asset_server.load("bevy/rpg/props/generic-rpg-barrel01.png");
    commands
        .spawn((
            Resource::Food(3),
            Sprite {
                image: barrel,
                ..default()
            },
            Transform::from_scale(Vec3::splat(6.0)),
            Pickable::default(),
        ))
        .observe(check_click);

    let home = asset_server.load("bevy/rpg/chars/vendor/generic-rpg-vendor.png");
    let home_transform = Transform::from_xyz(-300.0, 0.0, 0.0);
    commands
        .spawn((
            HomeTag,
            Sprite {
                image: home,
                ..default()
            },
            home_transform.with_scale(Vec3::splat(3.0)),
            // Pickable::default(),
        ))
        .observe(check_click);

    // Adapted from:
    // https://bevy.org/examples/picking/sprite-picking/
    let gabe = asset_server.load("bevy/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 7, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        WorkerState::AtHome,
        Sprite::from_atlas_image(
            gabe,
            TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: animation_indices.first,
            },
        ),
        Visibility::Hidden,
        home_transform.with_scale(Vec3::splat(6.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Pickable::default(),
    ));
}

fn check_click(
    _click: Trigger<Pointer<Pressed>>,
    mut resource_queue: EventWriter<WorkerRequest>,
    query: Query<(&Resource, &mut Transform)>,
) {
    // TODO: do we give worker resource type?
    // Maybe we want it to be hidden until the worker returns.
    // If we only care about the player *not* seeing
    // the resource (e.g., cloudly level/mechanic),
    // then we can store it internally in the resource.
    // If the worker prepares ahead for a specific resource,
    // however, *then* we may need to refactor.
    if let Some((resource, transform)) = query.iter().next() {
        resource_queue.write(WorkerRequest::new(Some(*resource), transform.translation));
    }
}

fn activate_available_worker(
    mut commands: Commands,
    mut workers: Query<(Entity, &mut WorkerState)>,
    mut task_queue: EventReader<WorkerRequest>,
) {
    if let Some(task) = task_queue.read().next() {
        for (worker_id, mut worker_state) in &mut workers {
            match *worker_state {
                WorkerState::AtHome | WorkerState::Passive => {
                    *worker_state = WorkerState::ToTarget;
                    commands.entity(worker_id).insert(WorkerTask::from(*task));
                    break;
                }
                _ => continue,
            }
        }
    }
}

fn show_worker(mut query: Query<(&WorkerState, &mut Visibility)>) {
    if let Some((WorkerState::ToTarget | WorkerState::Passive, mut visibility)) =
        query.iter_mut().next()
    {
        *visibility = Visibility::Visible
    }
}

// TODO: add resources to home!
fn update_home(mut query: Query<(&WorkerState, &mut Sprite, &mut Visibility)>) {
    if let Some((state, mut sprite, mut visibility)) = query.iter_mut().next()
        && let WorkerState::AtHome = *state
    {
        *visibility = Visibility::Hidden;
        // TODO: setup orientation
        // for general paths.
        sprite.flip_x = false;
        sprite.flip_y = false;
    }
}

fn move_worker(
    mut worker_query: Query<(&mut Transform, &mut WorkerState, &WorkerTask)>,
    home_query: Query<(&HomeTag, &Transform), Without<WorkerState>>,
) {
    if let Some((mut transform, mut state, task)) = worker_query.iter_mut().next() {
        let (_, home_transform) = home_query.iter().next().unwrap();

        // TODO: use path following here!
        match *state {
            // TODO: refactor with a Target struct!
            WorkerState::ToTarget => {
                if transform.translation == task.position {
                    // TODO: update resource that
                    // it's being worked on!
                    *state = WorkerState::AtTarget;
                } else {
                    transform.translation.x += 4.;
                    transform.translation.y += 0.;
                }
            }
            WorkerState::ToHome => {
                if transform.translation == home_transform.translation {
                    // TODO: update resource that
                    // it's being worked on!
                    *state = WorkerState::AtHome;
                } else {
                    transform.translation.x -= 4.;
                    transform.translation.y -= 0.;
                }
            }
            _ => {}
        }
    }
}

fn complete_task(mut workers: Query<(&mut WorkerState, &WorkerTask)>, resource: Query<&Resource>) {
    for (mut worker_state, worker_task) in &mut workers {
        // TODO: actually use time component!
        if let WorkerState::AtTarget = *worker_state {
            *worker_state = WorkerState::ToHome;
        }
    }
}

fn animate_worker(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &WorkerState,
        &mut Sprite,
    )>,
) {
    for (indices, mut timer, worker_state, mut sprite) in &mut query {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        timer.tick(time.delta());

        match worker_state {
            WorkerState::Passive => texture_atlas.index = 0,
            WorkerState::ToTarget => {
                if timer.just_finished() {
                    texture_atlas.index = if texture_atlas.index == indices.last {
                        indices.first
                    } else {
                        texture_atlas.index + 1
                    };
                }
            }
            WorkerState::ToHome => {
                if timer.just_finished() {
                    texture_atlas.index = if texture_atlas.index == indices.last {
                        indices.first
                    } else {
                        texture_atlas.index + 1
                    };
                }

                sprite.flip_x = true;
            }

            _ => (),
        }
    }
}
