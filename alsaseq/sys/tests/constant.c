// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ALSASEQ_CLIENT_TYPE_KERNEL);
    PRINT_CONSTANT((gint) ALSASEQ_CLIENT_TYPE_NONE);
    PRINT_CONSTANT((gint) ALSASEQ_CLIENT_TYPE_USER);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_ERROR_FAILED);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_ERROR_INVALID_DATA_TYPE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_ERROR_INVALID_LENGTH_MODE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_ERROR_INVALID_TSTAMP_MODE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_LENGTH_MODE_FIXED);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_LENGTH_MODE_POINTER);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_LENGTH_MODE_VARIABLE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_PRIORITY_MODE_HIGH);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_PRIORITY_MODE_NORMAL);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TIME_MODE_ABS);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TIME_MODE_REL);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TSTAMP_MODE_REAL);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TSTAMP_MODE_TICK);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_BOUNCE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CHANPRESS);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CLIENT_CHANGE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CLIENT_EXIT);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CLIENT_START);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CLOCK);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CONTINUE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CONTROL14);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_CONTROLLER);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_ECHO);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_KEYPRESS);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_KEYSIGN);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_NONE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_NONREGPARAM);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_NOTE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_NOTEOFF);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_NOTEON);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_OSS);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PGMCHANGE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PITCHBEND);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PORT_CHANGE);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PORT_EXIT);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PORT_START);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PORT_SUBSCRIBED);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_PORT_UNSUBSCRIBED);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_QFRAME);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_QUEUE_SKEW);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_REGPARAM);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_RESET);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_RESULT);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SENSING);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SETPOS_TICK);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SETPOS_TIME);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SONGPOS);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SONGSEL);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_START);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_STOP);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SYSEX);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_SYSTEM);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_TEMPO);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_TICK);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_TIMESIGN);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_TUNE_REQUEST);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR0);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR1);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR2);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR3);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR4);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR5);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR6);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR7);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR8);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR9);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR_VAR0);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR_VAR1);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR_VAR2);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR_VAR3);
    PRINT_CONSTANT((gint) ALSASEQ_EVENT_TYPE_USR_VAR4);
    PRINT_CONSTANT((guint) ALSASEQ_FILTER_ATTR_FLAG_BOUNCE);
    PRINT_CONSTANT((guint) ALSASEQ_FILTER_ATTR_FLAG_BROADCAST);
    PRINT_CONSTANT((guint) ALSASEQ_FILTER_ATTR_FLAG_MULTICAST);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_APPLICATION);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_HARDWARE);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_MIDI_GENERIC);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_MIDI_GM);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_MIDI_GM2);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_MIDI_GS);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_MIDI_MT32);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_MIDI_XG);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_PORT);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_SOFTWARE);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_SPECIFIC);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_ATTR_FLAG_SYNTHESIZER);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_CAP_FLAG_DUPLEX);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_CAP_FLAG_NO_EXPORT);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_CAP_FLAG_READ);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_CAP_FLAG_SUBS_READ);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_CAP_FLAG_SUBS_WRITE);
    PRINT_CONSTANT((guint) ALSASEQ_PORT_CAP_FLAG_WRITE);
    PRINT_CONSTANT((gint) ALSASEQ_QUERY_SUBSCRIBE_TYPE_READ);
    PRINT_CONSTANT((gint) ALSASEQ_QUERY_SUBSCRIBE_TYPE_WRITE);
    PRINT_CONSTANT((gint) ALSASEQ_QUEUE_TIMER_TYPE_ALSA);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_DEST);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_DEST_CHANNEL);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_EVENT_TYPE);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_IGNORE_OFF);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_INPUT);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_OUTPUT);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_TAG_MATCH);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_TIME_AFTER);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_TIME_BEFORE);
    PRINT_CONSTANT((guint) ALSASEQ_REMOVE_FILTER_FLAG_TIME_TICK);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_ADDRESS_BROADCAST);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_ADDRESS_SUBSCRIBERS);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_ADDRESS_UNKNOWN);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_CLIENT_ID_DUMMY);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_CLIENT_ID_OSS);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_CLIENT_ID_SYSTEM);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_PORT_ID_SYSTEM_ANNOUNCE);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_PORT_ID_SYSTEM_TIMER);
    PRINT_CONSTANT((gint) ALSASEQ_SPECIFIC_QUEUE_ID_DIRECT);
    PRINT_CONSTANT((gint) ALSASEQ_USER_CLIENT_ERROR_EVENT_UNDELIVERABLE);
    PRINT_CONSTANT((gint) ALSASEQ_USER_CLIENT_ERROR_FAILED);
    PRINT_CONSTANT((gint) ALSASEQ_USER_CLIENT_ERROR_PORT_PERMISSION);
    PRINT_CONSTANT((gint) ALSASEQ_USER_CLIENT_ERROR_QUEUE_PERMISSION);
    return 0;
}