import cv2
import mediapipe as mp

def main():
    # 初始化MediaPipe的Pose模型
    mp_pose = mp.solutions.pose
    pose = mp_pose.Pose(static_image_mode=False, min_detection_confidence=0.5, min_tracking_confidence=0.5)

    # 读取输入视频
    video_path = "/Users/liuluopeng/Movies/ccc.mp4"
    cap = cv2.VideoCapture(video_path)

    # 获取视频的宽度和高度
    width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
    height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

    # 设置输出视频的编码器和写入对象
    fourcc = cv2.VideoWriter_fourcc(*'XVID')
    output_video = cv2.VideoWriter('output_video.avi', fourcc, 30.0, (width, height))

    while cap.isOpened():
        ret, frame = cap.read()
        if not ret:
            break

        # 将BGR帧转换为RGB帧
        frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)

        # 使用MediaPipe进行骨架活动识别
        results = pose.process(frame_rgb)

        # 绘制骨架连接线
        if results.pose_landmarks is not None:
            mp_drawing = mp.solutions.drawing_utils
            mp_drawing.draw_landmarks(frame, results.pose_landmarks, mp_pose.POSE_CONNECTIONS,
                                      landmark_drawing_spec=mp_drawing.DrawingSpec(color=(0, 255, 0),
                                                                                    thickness=20,
                                                                                    circle_radius=2),
                                      connection_drawing_spec=mp_drawing.DrawingSpec(color=(255, 0, 0),
                                                                                      thickness=20))

        # 将RGB帧转换回BGR帧
        # frame_output = cv2.cvtColor(frame, cv2.COLOR_RGB2BGR)
        frame_output = cv2.cvtColor(frame.copy(), cv2.COLOR_RGB2BGR)

        # 将输出帧写入输出视频
        output_video.write(frame_output)

        # 显示输出帧
        cv2.imshow('Output', frame_output)
        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    # 释放资源
    cap.release()
    output_video.release()
    cv2.destroyAllWindows()

if __name__ == "__main__":
    main()
