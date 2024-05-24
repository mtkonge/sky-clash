void RfidScanner::begin() {
  Serial.println(String("Loading ") + this->pins.to_string());
  int attempts = 0;
  Serial.println(String("attempt ") + attempts);
  Serial.println("RFID begin()...");
  bool success = this->rfid.begin();
  if (!success) {
    Serial.println("RFID begin() failed");
  }
  Serial.println("RFID begin() ok");
  Serial.println("RFID getFirmwareVersion()...");
  uint32_t version = this->rfid.getFirmwareVersion();
  Serial.println("RFID getFirmwareVersion() yielded");
  while (version == 0) {
    Serial.println("version == 0");
    delay(500);
    Serial.println("RFID begin()...");
    bool success = this->rfid.begin();
    if (!success) {
      Serial.println("RFID begin() failed");
    }
    Serial.println("RFID begin() ok");
    Serial.println("RFID getFirmwareVersion()...");
    version = this->rfid.getFirmwareVersion();
    Serial.println("RFID getFirmwareVersion() yielded");
    Serial.println(String("attempt ") + attempts);
    attempts++;
    if (attempts > 20) {
      while (1) {}
    }
  }
  Serial.println(String("version = ") + version);

}


uint32_t RfidScanner::read(uint16_t timeout_ms) {
  uint8_t success;
  uint8_t uid[] = { 0, 0, 0, 0, 0, 0, 0 };
  uint8_t uid_length;

  success = this->rfid.readPassiveTargetID(PN532_MIFARE_ISO14443A, uid, &uid_length, timeout_ms);

  if (!success) {
    return 0;
  }
  if (uid_length > 4) {
    Serial.println("RfidScanner: Invalid RFID, >4 bytes");
    return 0;
  }
  auto rfid = byte_array_to_int(uid, uid_length);
  return rfid;
}



uint32_t byte_array_to_int(const uint8_t* bytes, uint8_t length) {
  uint32_t value = 0;
  for (uint8_t i=0; i < length; i++) {
    value |= (bytes[i] << i * 8);
  }
  return value;
}
