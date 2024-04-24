void RfidScanner::begin() {

  bool success = this->rfid.begin();
  if (!success) {
    Serial.println("RFID failed");
  }

  uint32_t version = this->rfid.getFirmwareVersion();
  while (version == 0) {
    Serial.println("version == 0");
    delay(500);
    bool success = this->rfid.begin();
    if (!success) {
      Serial.println("RFID failed");
    }
    version = this->rfid.getFirmwareVersion();
  }
  Serial.println(String("version = ") + version);

}

void RfidScanner::read() {
  uint8_t success;
  uint8_t uid[] = { 0, 0, 0, 0, 0, 0, 0 };
  uint8_t uidLength;

  success = this->rfid.readPassiveTargetID(PN532_MIFARE_ISO14443A, uid, &uidLength);

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
