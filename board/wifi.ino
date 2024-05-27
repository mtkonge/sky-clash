#include "env.h"
#include <SPI.h>
#include <WiFiNINA.h>

void Wifi::connect() {
  int status = WL_IDLE_STATUS;
  while (status != WL_CONNECTED) {
    Serial.print("Attempting to connect to network: ");
    Serial.println(ssid);
    status = WiFi.begin(ssid, pass);
    delay(5000);
  }
  Serial.print(String("Connected to ") + ssid + "!");
}

void Wifi::ping() {
  int pingResult;
  Serial.print("Pinging ");
  Serial.println(this->ip);
  pingResult = WiFi.ping(this->ip);

  if (pingResult >= 0) {
    Serial.print("SUCCESS! RTT = ");
    Serial.print(pingResult);
    Serial.println(" ms");
  } else {
    Serial.print("FAILED! Error code: ");
    Serial.println(pingResult);
  }
}

void Wifi::print_info() {
  Serial.print("SSID: ");
  Serial.println(WiFi.SSID());

  IPAddress ip = WiFi.localIP();
  Serial.print("IP Address: ");
  Serial.println(ip);

  long rssi = WiFi.RSSI();
  Serial.print("signal strength (RSSI):");
  Serial.print(rssi);
  Serial.println(" dBm");

}

String Wifi::get(const String& path) {
  if (client.connect(this->ip, this->port)) {
    client.println(String("GET ") + path + " HTTP/1.1");
    client.println(String("Host: ") + this->ip + ":" + this->port);
    client.println();
    while (client.available() == 0) {
      delay(100);
    }
    String response;
    int bytes_left = client.available();
    for (int i = 0; i < bytes_left; i++) {
      int byte = client.read();
      if (byte == -1) {
        break;
      }
      response += static_cast<char>(byte);
    }
    return response;
  } else {
    Serial.println(String("Could not get ") + this->ip + ":" + this->port + ", unresolved hostname" );
    return "Unresolved hostname";
  }
}

String Wifi::post(const String& path, const String& data) {
  if (client.connect(this->ip, this->port)) {
    client.println(String("POST ") + path + " HTTP/1.1");
    client.println(String("Host: ") + this->ip + ":" + this->port);
    client.println("Content-Type: application/json");
    client.print("Content-Length: ");
    client.println(data.length());
    client.println();
    client.print(data);
    while (client.available() == 0) {
      delay(100);
    }
    String response;
    int bytes_left = client.available();
    for (int i = 0; i < bytes_left; i++) {
      int byte = client.read();
      if (byte == -1) {
        break;
      }
      response += static_cast<char>(byte);
    }
    return response;
  } else {
    Serial.println(String("Could not post to ") + this->ip + ":" + this->port + ", unresolved hostname" );
    return "Unresolved hostname";
  }
}
