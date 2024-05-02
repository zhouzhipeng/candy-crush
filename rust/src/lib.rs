use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::engine::Sprite2D;


#[derive(GodotClass )]
#[class(base=Sprite2D)]
struct Player {
    #[export]
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
    #[export]
    candy: Option<Gd<Sprite2D>> ,
}

use godot::engine::ISprite2D;

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("H113366ello, world!"); // Prints to the Godot console

        Self {
            speed: 100.0,
            angular_speed: std::f64::consts::PI,
            base,
            candy: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("candy  : {:?}", self.candy);
        let callable = Callable::from_fn("rust_on_candy_vis", |args: &[&Variant]| {
            godot_print!("rust_on_candy_vis called");
            Ok(Variant::from(0))
        });
        self.candy.as_mut().unwrap().connect("visibility_changed".into(),  callable);
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}


#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        let speed = self.speed;
        self.base_mut().emit_signal("speed_increased".into(), &[Variant::from(speed)]);
    }

    #[signal]
    fn speed_increased(speed: f64){}

}
