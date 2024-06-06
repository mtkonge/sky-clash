#pragma once

#include "wifi.h"
#include <memory>

struct LedPins {
  int red, green, blue;
};

class Color {
  public:
  Color(uint8_t red, uint8_t green, uint8_t blue)
      : red(red)
      , green(green)
      , blue(blue)
  {
  }
  static Color white()
  {
    return { 255, 255, 255 };
  };
  void set_value_at_index(uint8_t color, int index);
  uint8_t red;
  uint8_t green;
  uint8_t blue;
};

class Leds {
  public:
  Leds(Wifi& client, LedPins left_pins, LedPins right_pins)
      : client(&client)
      , left_pins(left_pins)
      , right_pins(right_pins)
  {

    pinMode(left_pins.red, OUTPUT);
    pinMode(left_pins.green, OUTPUT);
    pinMode(left_pins.blue, OUTPUT);
    pinMode(right_pins.red, OUTPUT);
    pinMode(right_pins.green, OUTPUT);
    pinMode(right_pins.blue, OUTPUT);

    set_colors(Color::white(), Color::white());
  };

  bool can_update();
  void update(bool use_api);

  private:
  void set_colors(Color left, Color right);
  void parse_board_colors_response(String& response, Color& left, Color& right);

  Wifi* client;
  LedPins left_pins;
  LedPins right_pins;
  uint64_t millis_at_last_board_color_request = 0;
};
