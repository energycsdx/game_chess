#![warn( missing_docs )]

//! The sample which draw a chess board and GUI side panel with combobox.

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::input::system::exit_on_esc_system;

use bevy::window::WindowResizeConstraints;

const DISPLAY_HEIGHT : f32 = 600.0;
const DISPLAY_WIDTH : f32 = 800.0;

const DESK_HEIGHT : u8 = 8;
const DESK_WIDTH : u8 = 8;

///
/// Main.
///

fn main()
{
  App::build()
  .insert_resource( ClearColor( Color::rgb( 0.0, 0.0, 0.0 ) ) )
  .add_startup_system( setup.system() )
  .add_system( exit_on_esc_system.system() )
  .add_startup_stage( "game_setup", SystemStage::single( spawn_board.system() ) )
  .insert_resource( WindowDescriptor
  {
    title : "Spawn board".to_string(),
    width : DISPLAY_WIDTH,
    height : DISPLAY_HEIGHT,
    resizable : true,
    resize_constraints : WindowResizeConstraints
    {
      min_width : DISPLAY_WIDTH,
      min_height : DISPLAY_HEIGHT,
      ..Default::default()
    },
    ..Default::default()
  })
  .add_system_set_to_stage
  (
    CoreStage::PostUpdate,
    SystemSet::new()
    .with_system( position_translation.system() )
    .with_system( size_scaling.system() ),
  )
  .add_plugins( DefaultPlugins )
  .run();
}

///
/// Start setup, adding main resources.
///

fn setup
(
  mut commands : Commands,
  mut materials : ResMut<Assets<ColorMaterial>>,
)
{
  commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
  // commands.spawn_bundle( UiCameraBundle::default() );
  // add resource with materials for chess board
  commands.insert_resource( Materials
  {
    black : materials.add( Color::rgb( 0.30, 0.05, 0.0 ).into() ),
    white : materials.add( Color::rgb( 1.0, 1.0, 1.0 ).into() ),
  });
}

///
/// Struct for board position declaration.
///

#[derive( Default, Copy, Clone, Eq, PartialEq, Hash, Debug )]
struct Position
{
  x : i32,
  y : i32,
}

///
/// Struct to define size of chess board cell.
///

struct Size
{
  width : f32,
  height : f32,
}

impl Size
{
  fn cell( x : f32 ) -> Self
  {
    Self
    {
      width : x,
      height : x,
    }
  }
}

///
/// Struct to handle game materials.
///

struct Materials
{
  black : Handle< ColorMaterial >,
  white : Handle< ColorMaterial >,
}

///
/// Board as 64 sprites.
///

fn spawn_board
(
  mut commands : Commands,
  materials : Res<Materials>,
)
{
  for x in 0..DESK_WIDTH
  {
    for y in 0..DESK_HEIGHT
    {

      let material = if ( x + y + 1 ) % 2 == 0
      {
        &materials.white
      }
      else
      {
        &materials.black
      };

      commands.spawn_bundle( SpriteBundle
      {
        material : material.clone(),
        sprite : Sprite::new( Vec2::new( 10.0, 10.0 ) ),
        ..Default::default()
      })
      .insert( Position
      {
        x : x as i32,
        y : y as i32,
      })
      .insert( Size::cell( 0.95 ) )
      .id();
    }
  }
}

///
/// Post system which resizes board cells
///

fn size_scaling( windows : Res<Windows>, mut q : Query<( &Size, &mut Sprite )> )
{
  let window = windows.get_primary().unwrap();
  let mut width = window.width();
  let mut height = window.height();

  if width > height
  {
    width = height;
  }
  else
  {
    height = width;
  }

  for ( sprite_size, mut sprite ) in q.iter_mut()
  {
    sprite.size = Vec2::new
    (
      ( sprite_size.width / DESK_WIDTH as f32 * width as f32 ) * 0.9,
      ( sprite_size.height / DESK_HEIGHT as f32 * height as f32 ) * 0.9,
    );
  }
}

///
/// post system which sets board cells positions
///

fn position_translation
(
  windows: Res<Windows>,
  mut q : Query<( &Position, &mut Transform ) >,
)
{
  let window = windows.get_primary().unwrap();
  let mut width = window.width();
  let mut height = window.height();
  if width > height
  {
    width = height;
  }
  if width < height
  {
    height = width;
  }
  for ( pos, mut transform ) in q.iter_mut()
  {
    transform.translation = Vec3::new
    (
      0.05 * width + ( convert( pos.x as f32, width as f32, DESK_WIDTH as f32 ) - pos.x as f32 * 0.02 * width ),
      0.1 * height + ( convert( pos.y as f32, height as f32, DESK_HEIGHT as f32 ) - pos.y as f32 * 0.02 * height ),
      0.0,
    );
  }

  fn convert( pos : f32, bound_window : f32, bound_game : f32 ) -> f32
  {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - ( bound_window / 2.0 ) + ( tile_size / 2.0 )
  }

}
