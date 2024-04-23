#pragma once
#include <WiFiNINA.h>

class Wifi {
  public:
    Wifi(IPAddress ip, int port)
      : ip(ip)
      , port(port)
    {}

    void connect();
    void print_info();
    void ping();
    String post(const String& path, const String& data);

  private:
    IPAddress ip;
    int port;
    WiFiClient client;
};


