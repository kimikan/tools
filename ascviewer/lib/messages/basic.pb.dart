// ignore_for_file: invalid_language_version_override

import 'dart:async';
import 'dart:typed_data';
import 'package:rinf/rinf.dart';

//
//  Generated code. Do not modify.
//  source: basic.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// [RINF:DART-SIGNAL]
class SmallText extends $pb.GeneratedMessage {void sendSignalToRust() {
  sendDartSignal(
    0,
    this.writeToBuffer(),
    Uint8List(0),
  );
}

  factory SmallText({
    $core.String? text,
  }) {
    final $result = create();
    if (text != null) {
      $result.text = text;
    }
    return $result;
  }
  SmallText._() : super();
  factory SmallText.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SmallText.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SmallText', package: const $pb.PackageName(_omitMessageNames ? '' : 'basic'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SmallText clone() => SmallText()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SmallText copyWith(void Function(SmallText) updates) => super.copyWith((message) => updates(message as SmallText)) as SmallText;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SmallText create() => SmallText._();
  SmallText createEmptyInstance() => create();
  static $pb.PbList<SmallText> createRepeated() => $pb.PbList<SmallText>();
  @$core.pragma('dart2js:noInline')
  static SmallText getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SmallText>(create);
  static SmallText? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => clearField(1);
}

/// [RINF:RUST-SIGNAL]
class SmallNumber extends $pb.GeneratedMessage {static Stream<RustSignal<SmallNumber>> rustSignalStream =
    smallNumberController.stream.asBroadcastStream();

  factory SmallNumber({
    $core.int? number,
  }) {
    final $result = create();
    if (number != null) {
      $result.number = number;
    }
    return $result;
  }
  SmallNumber._() : super();
  factory SmallNumber.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SmallNumber.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SmallNumber', package: const $pb.PackageName(_omitMessageNames ? '' : 'basic'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'number', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SmallNumber clone() => SmallNumber()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SmallNumber copyWith(void Function(SmallNumber) updates) => super.copyWith((message) => updates(message as SmallNumber)) as SmallNumber;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SmallNumber create() => SmallNumber._();
  SmallNumber createEmptyInstance() => create();
  static $pb.PbList<SmallNumber> createRepeated() => $pb.PbList<SmallNumber>();
  @$core.pragma('dart2js:noInline')
  static SmallNumber getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SmallNumber>(create);
  static SmallNumber? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get number => $_getIZ(0);
  @$pb.TagNumber(1)
  set number($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearNumber() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');

final smallNumberController = StreamController<RustSignal<SmallNumber>>();
