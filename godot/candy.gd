extends Area2D
@export
var p: Player
func _on_input_event(viewport: Node, event: InputEvent, shape_idx: int) -> void:
	#print(event)
	if event.is_action_pressed("click"):
		print("click")

		p.increase_speed(10)


func _on_player_speed_increased(speed: float) -> void:
	print("from rust, speed: "+ str(speed))
