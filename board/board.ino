#include <SPI.h>
#include <WiFiNINA.h>
#include "env.h"

int status = WL_IDLE_STATUS;
int pingResult;

WiFiClient client;

IPAddress server(192,168,132,183);


void setup() {
  Serial.begin(9600);

  while (status != WL_CONNECTED) {
    Serial.print("Attempting to connect to network: ");
    Serial.println(ssid);
    status = WiFi.begin(ssid, pass);
    delay(10000);
  }
  Serial.print("You're connected to the network: ");
  Serial.println(ssid);

  printWifiStatus();

  ping("192.168.132.183");

  post_request();
}

void ping(char* ip) {
  Serial.print("Pinging ");
  Serial.println(ip);
  pingResult = WiFi.ping(ip);

  if (pingResult >= 0) {
    Serial.print("SUCCESS! RTT = ");
    Serial.print(pingResult);
    Serial.println(" ms");
  } else {
    Serial.print("FAILED! Error code: ");
    Serial.println(pingResult);
  }
}



void printWifiStatus() {
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
void get_google_html() {
  Serial.println("\nStarting connection to server...");
  if (client.connect("www.google.com", 80)) {
    Serial.println("connected to server");
    client.println("GET /search?q=arduino HTTP/1.1");
    client.println("Host: www.google.com");
    client.println("Connection: close");
    client.println();
  }
}


void post_request() {
  String data = "{\"rfid\": \"aiojfejiofeq\", \"hero_type\": 4}";
  Serial.println("WE ARE MAKING A POST REQUEST NOBODY BREATH");
  if (client.connect("192.168.132.183", 8080)) {
    Serial.println("WE ARE IN !!!!");
    client.println("POST /create_hero HTTP/1.1");
    client.println("Host: 192.168.132.183:8080");
    client.println("Content-Type: application/json");
    client.print("Content-Length: ");
    client.println(data.length());
    client.println();
    client.print(data);
  } else {
    Serial.println("Could not connect");
  }
}


void loop() {

  while (client.available()) {
    char c = client.read();
    Serial.write(c);
  }




}