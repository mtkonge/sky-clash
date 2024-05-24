#pragma once
#include <Adafruit_PN532.h>

struct RfidConnection {
  virtual ~RfidConnection() {};

  virtual Adafruit_PN532 build() = 0;
};

struct RfidI2C final : public RfidConnection {
  RfidI2C(int irq_pin, int rsto_pin, TwoWire* wire)
    : irq_pin(irq_pin)
    , rsto_pin(rsto_pin)
    , wire(wire)
  {}

  inline Adafruit_PN532 build() override {
    return Adafruit_PN532(this->irq_pin, this->rsto_pin, this->wire);
  }

  int irq_pin;
  int rsto_pin;
  TwoWire* wire;
};

struct RfidSPI final : public RfidConnection {
  RfidSPI(uint8_t reset_pin, HardwareSerial* serial)
    : reset_pin(reset_pin)
    , serial(serial)
  {}

  inline Adafruit_PN532 build() override {
    return Adafruit_PN532(this->reset_pin, this->serial);
  }

  uint8_t reset_pin;
  HardwareSerial* serial;
};

struct RfidPins {
  int sda;
  int scl;
  int irq;
  int rsto;

  String to_string() const;
};

String RfidPins::to_string() const {
  String result;
  result += "RFID { sda: ";
  result += this->sda;
  result += ", scl: ";
  result += this->scl;
  result += ", irq: ";
  result += this->irq;
  result += ", rsto: ";
  result += this->rsto;
  result += " }";
  return result;
}

class RfidScanner {
  public:
    template<typename T>
    RfidScanner(T connection, RfidPins pins)
      : rfid(connection.build())
      , pins(pins)
    {
    }

    void begin();
    // returns 0 on failure
    uint32_t read(uint16_t timeout_ms);

  private:
    Adafruit_PN532 rfid;
    RfidPins pins;
};