class_name ChatHandshake;

var username: String;
var room_id: int;
var password: String;

func _init(username: String, room_id: int, password: String):
	self.username = username;
	self.room_id = room_id;
	self.password = password;

func json():
	var json = JSON.new();
	var data = {
		"username": username,
		"room_id": room_id,
		"password": password
	}
	return json.stringify(data);
