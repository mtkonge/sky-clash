#include <WiFiNINA.h>
#include "env.h"

char hostName[] = "www.google.com";
int status = WL_IDLE_STATUS;
int pingResult;

void setup() {

  Serial.begin(9600);
  // attempt to connect to Wifi network:

  // Wifi ssid and pass needs to be defined in env.h

  while (status != WL_CONNECTED) {

    Serial.print("Attempting to connect to network: ");

    Serial.println(ssid);

    status = WiFi.begin(ssid, pass);


    delay(10000);

  }

  Serial.print("You're connected to the network: ");
  Serial.println(ssid);


}

void loop() {
  Serial.print("Pinging ");

  Serial.println(hostName);


  pingResult = WiFi.ping(hostName);

  if (pingResult >= 0) {

    Serial.print("SUCCESS! RTT = ");

    Serial.print(pingResult);

    Serial.println(" ms");

  } else {

    Serial.print("FAILED! Error code: ");

    Serial.println(pingResult);

  }

  delay(5000);

}