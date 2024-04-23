#include <Adafruit_PN532.h>

#define IRQ_PIN 1
#define RSTO_PIN 0

Adafruit_PN532 rfid(IRQ_PIN, RSTO_PIN, &Wire);

void setup() {
  Serial.begin(9600);

  while (!Serial) {
    delay(100);
  }

  Serial.println("Start");

  bool success = rfid.begin();
  if (!success) {
    Serial.println("RFID failed");
  }

  uint32_t version = rfid.getFirmwareVersion();
  while (version == 0) {
    Serial.println("version == 0");
    delay(500);
    bool success = rfid.begin();
    if (!success) {
      Serial.println("RFID failed");
    }
    version = rfid.getFirmwareVersion();
  }
  Serial.println(String("version = ") + version);

  // rfid.setPassiveActivationRetries(0xFF);
}

int slurs = 0;

void loop() {
  uint8_t success;
  uint8_t uid[] = { 0, 0, 0, 0, 0, 0, 0 };
  uint8_t uidLength;

  success = rfid.readPassiveTargetID(PN532_MIFARE_ISO14443A, uid, &uidLength);

  Serial.println(String("slurs = ") + slurs);
  slurs += 1;

  if (success) {
    Serial.println("Found an ISO14443A card");
    Serial.print(String("  UID Length: ") + uidLength + " bytes");
    Serial.print("  UID Value: ");
    for (uint8_t i=0; i < uidLength; i++) {
      Serial.print(" 0x");
      Serial.print(uid[i], HEX);
    }
    Serial.println("");
    delay(100);
  } else {
    Serial.println("Failed");
    delay(500);
  }
}
