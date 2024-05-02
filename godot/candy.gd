extends Area2D


func _on_input_event(viewport: Node, event: InputEvent, shape_idx: int) -> void:
	#print(event)
	if event.is_action_pressed("click"):
		print("click")
