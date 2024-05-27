class_name ChatMessage;

var username: String;
var room_id: int;
var message: String;

func _init(username: String, room_id: int, password: String):
	self.username = username;
	self.room_id = room_id;
	self.message = message;

func json():
	var json = JSON.new();
	var data = {
		"username": username,
		"room_id": room_id,
		"message": message
	}
	return json.stringify(data);
