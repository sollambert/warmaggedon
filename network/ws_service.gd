extends Node

var socket = WebSocketPeer.new();
@export var ws_base_url = "ws://127.0.0.1:3001";
var username: String;
var password: String;
var first_connected = true;

# Called when the node enters the scene tree for the first time.
func _ready():
	#join_room("", "testing1", "password");
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	socket.poll();
	var state = socket.get_ready_state();
	if state == WebSocketPeer.STATE_OPEN:
		if first_connected:
			first_connected = !first_connected;
			var message = ChatHandshake.new(username, password);
			send_greeting(message);
		while socket.get_available_packet_count():
			print("Message: ", socket.get_packet().get_string_from_ascii());
	elif state == WebSocketPeer.STATE_CLOSING:
		# do closing, do not break polling
		pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = socket.get_close_code();
		var reason = socket.get_close_reason();
		print("WebSocket closed with code: %d, reason %s. Clean: %s" %[code, reason, code != -1]);
		set_process(false);
	pass
	
func join_room(id: String, username: String, password: String):
	socket.connect_to_url(ws_base_url + "/room/join/" + id);
	self.username = username;
	self.password = password;
	pass;

func send_greeting(msg: ChatHandshake):
	socket.send_text(msg.json());

func send_message(msg: ChatMessage):
	socket.send_text(msg.json());
	pass
