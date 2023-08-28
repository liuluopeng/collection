<template>
  <n-data-table :columns="ppcolumns" :data="ppdata.table" :pagination="pagination" :bordered="false" />

</template>

<script lang="ts">
import { h, defineComponent, onMounted, reactive } from "vue";
import { NButton, useMessage } from "naive-ui";
import type { DataTableColumns } from "naive-ui";
import { getBiliDashboard } from "@/api/bili/bili";



const createPPColumns = () => {
  return [
    {
      title: "Title",
      key: "title",
    },
    {
      title: "Introduction",
      key: "intro",
    }
  ]
}




let ppdata = reactive({
   table: [],
})




export default defineComponent({
  setup() {
    const message = useMessage();



    function to_duration(morse_list) {
      var res = [];
      for(var i=0;i<morse_list.length;i++) {
        if (morse_list[i] == 0) {
          res.push(1);
        } else if (morse_list[i] == 1) {
          res.push(3);
        }
      }
      return res;
    }

    function makeBibibi(duration_list) {

      var i = 0;

      var unitDuration = 50;
      function playNextSound() {

        if (i < duration_list.length) {
          console.log(i, duration_list[i]);

          const audioCtx = new (window.AudioContext || window.AudioContext)();
          const oscillator = audioCtx.createOscillator();
          oscillator.type = 'sine'; // 正弦波
          oscillator.frequency.setValueAtTime(550, audioCtx.currentTime); // 设置频率为 440 Hz
          oscillator.connect(audioCtx.destination); // 将正弦波连接到音频输出

          oscillator.start(); // 开始播放
          oscillator.stop(audioCtx.currentTime + (duration_list[i]*unitDuration)/1000 ); // 停止播放

          setTimeout(() => {
            i += 1;
            playNextSound(); // 播放下一个声音
          }, unitDuration * 3 +  (duration_list[i]*unitDuration) ); // 等待声音播放完毕后再停止播放

        }
      }

      playNextSound();

    }


    return {
      ppdata,
      ppcolumns: createPPColumns(),
      pagination: false as const,
      onMounted: onMounted(async () => {
        ppdata.table = await getBiliDashboard();
        console.log(ppdata.table)

        var morse_list = [0,1,0,1]
        var morse_list = [1,0,1,0]
        console.log(morse_list.length)
        var duration_list = to_duration(morse_list);

        makeBibibi(duration_list);
      }),
  };
  }
});
</script>

