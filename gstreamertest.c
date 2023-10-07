#include <gst/gst.h>

int main(int argc, char *argv[]) {
    GstElement *pipeline, *video_source, *audio_source, *muxer, *sink;
    GstBus *bus;
    GstMessage *msg;

    /* Initialize GStreamer */
    gst_init (&argc, &argv);

    /* Create the elements */
    video_source = gst_element_factory_make ("filesrc", "video_source");
    audio_source = gst_element_factory_make ("filesrc", "audio_source");
    muxer = gst_element_factory_make ("mp4mux", "muxer");
    sink = gst_element_factory_make ("filesink", "sink");

    /* Create the empty pipeline */
    pipeline = gst_pipeline_new ("test-pipeline");

    if (!pipeline || !video_source || !audio_source || !muxer || !sink) {
        g_printerr ("Not all elements could be created.\n");
        return -1;
    }

    /* Configure elements */
    g_object_set (video_source, "location", "input_video.mp4", NULL);
    g_object_set (audio_source, "location", "input_audio.aac", NULL);
    g_object_set (sink, "location", "output.mp4", NULL);

    /* Link the elements */
    gst_bin_add_many (GST_BIN (pipeline), video_source, audio_source, muxer, sink, NULL);
    if (gst_element_link_many (video_source, muxer, sink, NULL) != TRUE ||
        gst_element_link_many (audio_source, muxer, NULL) != TRUE) {
        g_printerr ("Elements could not be linked.\n");
        gst_object_unref (pipeline);
        return -1;
    }

    /* Start playing */
    gst_element_set_state (pipeline, GST_STATE_PLAYING);

    /* Wait until error or EOS */
    bus = gst_element_get_bus (pipeline);
    msg = gst_bus_timed_pop_filtered (bus, GST_CLOCK_TIME_NONE,
        GST_MESSAGE_ERROR | GST_MESSAGE_EOS);

    /* Clean up */
    if (msg != NULL)
        gst_message_unref (msg);
    gst_object_unref (bus);
    gst_element_set_state (pipeline, GST_STATE_NULL);
    gst_object_unref (pipeline);

    return 0;
}
