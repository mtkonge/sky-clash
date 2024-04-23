#include <Adafruit_PN532.h>

class RfidScanner {
  public:
    RfidScanner(int irq_pin, int rsto_pin)
      : irq_pin(irq_pin)
      , rsto_pin(rsto_pin)
    {
      rfid(IRQ_PIN, RSTO_PIN, &Wire);
    }

    void begin();
    void read();

  private:
    int irq_pin;
    int rsto_pin;
    Adafruit_PN532 rfid;
};

RfidScanner::begin() {

}

RfidScanner::read() {

}
