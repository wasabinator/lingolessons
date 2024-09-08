#!/bin/sh
cflags=$(pkg-config --cflags gtk4)
ldflags=$(pkg-config --libs gtk4)
echo headers = gtk/gtk.h > cinterop/gtk4.def #gtk/gtkunixprint.h gtk/gtkmediastream.h
echo headerFilter = gtk/* gtk/deprecated/* gtk/css/*  >> cinterop/gtk4.def
echo compilerOpts = ${cflags} >> cinterop/gtk4.def
echo linkerOpts = ${ldflags} >> cinterop/gtk4.def
echo "
---

static inline gulong g_signal_connect(gpointer instance, 
                                      const gchar *detailed_signal, 
                                      c_handler, 
                                      data) 
{
    return g_signal_connect_data((instance), (detailed_signal), (c_handler), (data), NULL, (GConnectFlags) 0);
}
" >> cinterop/gtk4.def

