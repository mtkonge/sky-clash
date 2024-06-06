void Color::set_value_at_index(uint8_t color, int index) {
  switch (index) {
  case 0:
    red = color;
    break;
  case 1:
    green = color;
    break;
  case 2:
    blue = color;
    break;
  default:
    Serial.print("Color::set_value_at_index: unrecognized color index ");
    Serial.println(index);
    break;
  }
}

bool Leds::can_update() {
  uint64_t now = millis();
  uint64_t time_since_last = now - millis_at_last_board_color_request;
  return time_since_last > 1000;
}

void Leds::parse_board_colors_response(String& response, Color& left, Color& right) {
  uint32_t content_start = 0;
  uint32_t line_length = 0;
  while (content_start < response.length()) {
    if (response[content_start] == '\r' && response[content_start+1] == '\n') {
      content_start += 2;
      if (line_length == 0) {
        break;
      }
      line_length = 0;
    } else {
      line_length++;
      content_start++;
    }
  }
  if (content_start == response.length()) {
    Serial.println("unable to parse board colors: body was not received");
    Serial.println("--- response body begin");
    Serial.println(response);
    Serial.println("--- response body end");
    return;
  }
  uint8_t color = 0;
  int player_index = 0;
  int color_index = 0;
  for (uint32_t i = content_start + 1; i < response.length() - 1; i++) {
    unsigned char current = response[i];
    switch (current) {
      case '[':
        color_index = 0;
        color = 0;
        continue;
      case ',':
        if (player_index == 0) {
          left.set_value_at_index(color, color_index);
        } else {
          right.set_value_at_index(color, color_index);
        }
        color_index++;
        color = 0;
        continue;
      case ']':
        if (player_index == 0) {
          left.set_value_at_index(color, color_index);
        } else {
          right.set_value_at_index(color, color_index);
        }
        player_index++;
        i++;
        continue;
      default:
        if (current >= '0' && current <= '9') {
          color *= 10;
          color += (current - 48);
          continue;
        }
    }
    Serial.print("unhandled char when parsing: '");
    Serial.print(current);
    Serial.println("'");
  }
}

void Leds::set_colors(Color left, Color right) {
  analogWrite(left_pins.red, left.red);
  analogWrite(left_pins.green, left.green);
  analogWrite(left_pins.blue, left.blue);

  analogWrite(right_pins.red, right.red);
  analogWrite(right_pins.green, right.green);
  analogWrite(right_pins.blue, right.blue);
};

void Leds::update(bool use_api) {
  millis_at_last_board_color_request = millis();
  Color left(255, 255, 255);
  Color right(255, 255, 255);
  if (use_api) {
    String response = client->get("/board_colors");
    parse_board_colors_response(response, left, right);
  }
  set_colors(left, right);
}
