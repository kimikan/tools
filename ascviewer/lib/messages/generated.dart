import './frames.pb.dart' as frames;
import './basic.pb.dart' as basic;
// ignore_for_file: unused_import

import 'dart:typed_data';
import 'package:rinf/rinf.dart';

Future<void> initializeRust() async {
  await prepareInterface(handleRustSignal);
  startRustLogic();
}

Future<void> finalizeRust() async {
  stopRustLogic();
  await Future.delayed(const Duration(milliseconds: 10));
}

final signalHandlers = <int, void Function(Uint8List, Uint8List)>{
1: (Uint8List messageBytes, Uint8List binary) {
  final message = basic.SmallNumber.fromBuffer(messageBytes);
  final rustSignal = RustSignal(
    message,
    binary,
  );
  basic.smallNumberController.add(rustSignal);
},
3: (Uint8List messageBytes, Uint8List binary) {
  final message = frames.FramesResp.fromBuffer(messageBytes);
  final rustSignal = RustSignal(
    message,
    binary,
  );
  frames.framesRespController.add(rustSignal);
},
};

void handleRustSignal(int messageId, Uint8List messageBytes, Uint8List binary) {
  signalHandlers[messageId]!(messageBytes, binary);
}
