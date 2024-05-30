// ignore_for_file: invalid_language_version_override

import 'dart:async';
import 'dart:typed_data';
import 'package:rinf/rinf.dart';

//
//  Generated code. Do not modify.
//  source: frames.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// [RINF:DART-SIGNAL]
class FramesReq extends $pb.GeneratedMessage {void sendSignalToRust() {
  sendDartSignal(
    2,
    this.writeToBuffer(),
    Uint8List(0),
  );
}

  factory FramesReq() => create();
  FramesReq._() : super();
  factory FramesReq.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FramesReq.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FramesReq', package: const $pb.PackageName(_omitMessageNames ? '' : 'frames'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FramesReq clone() => FramesReq()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FramesReq copyWith(void Function(FramesReq) updates) => super.copyWith((message) => updates(message as FramesReq)) as FramesReq;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FramesReq create() => FramesReq._();
  FramesReq createEmptyInstance() => create();
  static $pb.PbList<FramesReq> createRepeated() => $pb.PbList<FramesReq>();
  @$core.pragma('dart2js:noInline')
  static FramesReq getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FramesReq>(create);
  static FramesReq? _defaultInstance;
}

class Signal extends $pb.GeneratedMessage {
  factory Signal({
    $core.String? sigName,
    $core.String? sigValue,
  }) {
    final $result = create();
    if (sigName != null) {
      $result.sigName = sigName;
    }
    if (sigValue != null) {
      $result.sigValue = sigValue;
    }
    return $result;
  }
  Signal._() : super();
  factory Signal.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Signal.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Signal', package: const $pb.PackageName(_omitMessageNames ? '' : 'frames'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sigName')
    ..aOS(2, _omitFieldNames ? '' : 'sigValue')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Signal clone() => Signal()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Signal copyWith(void Function(Signal) updates) => super.copyWith((message) => updates(message as Signal)) as Signal;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Signal create() => Signal._();
  Signal createEmptyInstance() => create();
  static $pb.PbList<Signal> createRepeated() => $pb.PbList<Signal>();
  @$core.pragma('dart2js:noInline')
  static Signal getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Signal>(create);
  static Signal? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sigName => $_getSZ(0);
  @$pb.TagNumber(1)
  set sigName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSigName() => $_has(0);
  @$pb.TagNumber(1)
  void clearSigName() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sigValue => $_getSZ(1);
  @$pb.TagNumber(2)
  set sigValue($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSigValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearSigValue() => clearField(2);
}

class FrameItem extends $pb.GeneratedMessage {
  factory FrameItem({
    $core.int? msgId,
    $core.String? msgName,
    $core.String? direction,
    $core.Iterable<Signal>? signals,
  }) {
    final $result = create();
    if (msgId != null) {
      $result.msgId = msgId;
    }
    if (msgName != null) {
      $result.msgName = msgName;
    }
    if (direction != null) {
      $result.direction = direction;
    }
    if (signals != null) {
      $result.signals.addAll(signals);
    }
    return $result;
  }
  FrameItem._() : super();
  factory FrameItem.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FrameItem.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FrameItem', package: const $pb.PackageName(_omitMessageNames ? '' : 'frames'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'msgId', $pb.PbFieldType.O3)
    ..aOS(2, _omitFieldNames ? '' : 'msgName')
    ..aOS(3, _omitFieldNames ? '' : 'direction')
    ..pc<Signal>(4, _omitFieldNames ? '' : 'signals', $pb.PbFieldType.PM, subBuilder: Signal.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FrameItem clone() => FrameItem()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FrameItem copyWith(void Function(FrameItem) updates) => super.copyWith((message) => updates(message as FrameItem)) as FrameItem;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FrameItem create() => FrameItem._();
  FrameItem createEmptyInstance() => create();
  static $pb.PbList<FrameItem> createRepeated() => $pb.PbList<FrameItem>();
  @$core.pragma('dart2js:noInline')
  static FrameItem getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FrameItem>(create);
  static FrameItem? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get msgId => $_getIZ(0);
  @$pb.TagNumber(1)
  set msgId($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMsgId() => $_has(0);
  @$pb.TagNumber(1)
  void clearMsgId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get msgName => $_getSZ(1);
  @$pb.TagNumber(2)
  set msgName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMsgName() => $_has(1);
  @$pb.TagNumber(2)
  void clearMsgName() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get direction => $_getSZ(2);
  @$pb.TagNumber(3)
  set direction($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDirection() => $_has(2);
  @$pb.TagNumber(3)
  void clearDirection() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<Signal> get signals => $_getList(3);
}

/// [RINF:RUST-SIGNAL]
class FramesResp extends $pb.GeneratedMessage {static Stream<RustSignal<FramesResp>> rustSignalStream =
    framesRespController.stream.asBroadcastStream();

  factory FramesResp({
    $core.Iterable<FrameItem>? frames,
  }) {
    final $result = create();
    if (frames != null) {
      $result.frames.addAll(frames);
    }
    return $result;
  }
  FramesResp._() : super();
  factory FramesResp.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FramesResp.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FramesResp', package: const $pb.PackageName(_omitMessageNames ? '' : 'frames'), createEmptyInstance: create)
    ..pc<FrameItem>(1, _omitFieldNames ? '' : 'frames', $pb.PbFieldType.PM, subBuilder: FrameItem.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FramesResp clone() => FramesResp()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FramesResp copyWith(void Function(FramesResp) updates) => super.copyWith((message) => updates(message as FramesResp)) as FramesResp;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FramesResp create() => FramesResp._();
  FramesResp createEmptyInstance() => create();
  static $pb.PbList<FramesResp> createRepeated() => $pb.PbList<FramesResp>();
  @$core.pragma('dart2js:noInline')
  static FramesResp getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FramesResp>(create);
  static FramesResp? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<FrameItem> get frames => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');

final framesRespController = StreamController<RustSignal<FramesResp>>();
