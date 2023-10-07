#include <libavformat/avformat.h>
int main(){
    AVFormatContext *ifmt_ctx_v = NULL, *ifmt_ctx_a = NULL,*ofmt_ctx = NULL;
    avformat_network_init();

    // must must must 否则返回一个特别大的负数
    av_register_all();

    AVDictionary *options = NULL;
    av_dict_set(&options, "referer", "https://www.bilibili.com", 0);
    av_dict_set(&options, "user_agent", "MyUserAgent/1.0", 0);
    //int ret = avformat_open_input(&ifmt_ctx_v, "https://xy1x197x251x175xy.mcdn.bilivideo.cn:8082/v1/resource/788850002_da2-1-30080.m4s?agrr=0&build=0&buvid=&bvc=vod&bw=272108&deadline=1695820674&e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M%3D&f=u_0_0&gen=playurlv2&logo=A0008000&mcdnid=16000384&mid=472837980&nbs=1&nettype=0&oi=828301138&orderid=0%2C3&os=mcdn&platform=pc&sign=9f4332&traceid=trHufSoyrYAayW_0_e_N&uipk=5&uparams=e%2Cuipk%2Cnbs%2Cdeadline%2Cgen%2Cos%2Coi%2Ctrid%2Cmid%2Cplatform&upsig=85f702d2420b0e7971c2cd9ae7a4ed39", NULL, &options);
    //fprintf(stderr, "Could not open input file '%s' (error '%s')\n","video", av_err2str(ret));
    int ret;
    printf("%d\n", avformat_open_input(&ifmt_ctx_a, "./input_video.mp4", 0, 0));
    //fprintf(stderr, "Could not open input file '%s' (error '%s')\n", "url", av_err2str(ret));
    //printf("%d\n", ret);
    printf("%d\n", avformat_open_input(&ifmt_ctx_a, "./input_audio.aac", 0, 0));


     printf("Format: %s\n", ifmt_ctx_v->iformat->name);
  
    // 打印流数量
    printf("Number of Streams: %d\n", ifmt_ctx_v->nb_streams);
     // 创建输出上下文


    avformat_alloc_output_context2(&ofmt_ctx, NULL, NULL, "output.mp4");



    AVStream *out_audio_stream = avformat_new_stream(ofmt_ctx, ifmt_ctx_a->streams[0]->codec->codec);
     avcodec_copy_context(out_audio_stream->codec, ifmt_ctx_a->streams[0]->codec);
    out_audio_stream->codec->codec_tag = 0;
    int audio_stream_index = -1, video_stream_index = -1;
    audio_stream_index = out_audio_stream->index;


    AVStream *out_video_stream = avformat_new_stream(ofmt_ctx, ifmt_ctx_v->streams[0]->codec->codec);
    avcodec_copy_context(out_video_stream->codec, ifmt_ctx_v->streams[0]->codec);
    out_video_stream->codec->codec_tag = 0;
    video_stream_index = out_video_stream->index;

        // 写输出文件的头部信息
    avio_open(&ofmt_ctx->pb, "output.mp4", AVIO_FLAG_WRITE);
    avformat_write_header(ofmt_ctx, NULL);

    AVPacket pkt;
    while (1) {
        AVFormatContext *src_ctx;
        // 这里简化了选择音频或视频的逻辑，实际应用中可能需要更复杂的逻辑
        if (audio_stream_index != -1) {
            src_ctx = ifmt_ctx_a;
            ret = av_read_frame(src_ctx, &pkt);
            if (ret < 0) {
                audio_stream_index = -1;
            } else {
                pkt.stream_index = audio_stream_index;
            }
        }
        if (video_stream_index != -1 && ret < 0) {
            src_ctx = ifmt_ctx_v;
            ret = av_read_frame(src_ctx, &pkt);
            if (ret < 0) {
                video_stream_index = -1;
            } else {
                pkt.stream_index = video_stream_index;
            }
        }
        if (ret < 0) {
            break;
        }

        // 写入输出文件
        av_interleaved_write_frame(ofmt_ctx, &pkt);
        av_packet_unref(&pkt);
    }

    av_write_trailer(ofmt_ctx);

    // 关闭所有文件和释放所有资源
    avformat_close_input(&ifmt_ctx_a);
    avformat_close_input(&ifmt_ctx_v);
    avio_close(ofmt_ctx->pb);
    avformat_free_context(ofmt_ctx);

    return 0;
}
