#pragma once
#include <Adafruit_PN532.h>

class RfidScanner {
  public:
    RfidScanner(int irq_pin, int rsto_pin)
      : irq_pin(irq_pin)
      , rsto_pin(rsto_pin)
      , rfid(irq_pin, rsto_pin, &Wire)
    {
    }

    void begin();
    // returns 0 on failure
    uint32_t read(uint16_t timeout_ms);

  private:
    int irq_pin;
    int rsto_pin;
    Adafruit_PN532 rfid;
};