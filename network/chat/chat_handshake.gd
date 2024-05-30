class_name ChatHandshake;

var username: String;
var password: String;

func _init(username: String, password: String):
	self.username = username;
	self.password = password;

func json():
	var json = JSON.new();
	var data = {
		"username": username,
		"password": password
	}
	return json.stringify(data);
