#ifndef OPTIONS_H
#define OPTIONS_H
#include <stdio.h>
#include <stdbool.h>
enum Cmd{
    UNUSED,
    DOWNLOAD,
    LOGIN,
};
enum LoginMethod{
    QRCODE,
    SMSCODE,
    PASSWORD,
};
struct Options{
    bool version;
    bool help;
    enum LoginMethod loginMethod;
};
#endif
