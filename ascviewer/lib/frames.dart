import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:myapp/messages/frames.pb.dart';
import 'package:rinf/rinf.dart';
import 'package:syncfusion_flutter_datagrid/datagrid.dart';

class FramesWidget extends StatefulWidget {
  const FramesWidget({super.key});

  @override
  State<FramesWidget> createState() => _FramesWidgetState();
}

class _FramesWidgetState extends State<FramesWidget> {
  get() async {
    FramesReq.create().sendSignalToRust();
    var s = FramesResp.rustSignalStream;

    return s.first;
  }

  Future<dynamic>? _future;

  late Map<String, double> columnWidths = {
    'MsgId': 150,
    'MsgName': 300,
    'Direction': 150,
    'Signals': double.nan
  };

  SignalsWidget _signalsWidget = SignalsWidget(List<Signal>.empty());

  final DataGridController _dataGridController = DataGridController();


  @override
  Widget build(BuildContext context) {
    _future ??= get();

    return FutureBuilder(
        future: _future!,
        builder: (ctx, snapshot) {
          if (snapshot.hasData) {
            var s = snapshot.data as RustSignal<FramesResp>;
            var s2 = s.message.frames.toString();
            return Row(
              children: [
                Expanded(
                  flex: 2,
                  child: Column(
                    children: [
                      Expanded(
                        child: SfDataGrid(
                          controller: _dataGridController,
                          selectionMode: SelectionMode.single,
                          onSelectionChanged: (r1, r2) {
                            if (r2.isNotEmpty) {
                              var item = s.message.frames[_dataGridController.selectedIndex];
                              _signalsWidget.updateSignals(item.signals);
                            }
                          },
                          allowColumnsDragging: true,
                          allowFiltering: true,
                          //rowHeight: 300,
                          rowsPerPage: 100,
                          onColumnResizeUpdate: (ColumnResizeUpdateDetails details) {
                            setState(() {
                              columnWidths[details.column.columnName] = details.width;
                            });
                            return true;
                          },
                          source: FrameDataSource(s.message.frames),
                          allowColumnsResizing: true,
                          columnWidthMode: ColumnWidthMode.lastColumnFill,
                          columnResizeMode: ColumnResizeMode.onResize,
                          columns: <GridColumn>[
                            GridColumn(
                                width: columnWidths["MsgId"]!,
                                columnName: 'MsgId',
                                label: Container(
                                    padding: EdgeInsets.all(16.0),
                                    alignment: Alignment.center,
                                    child: Text(
                                      'MsgID',
                                    ))),
                            GridColumn(
                                width: columnWidths["MsgName"]!,
                                columnName: 'MsgName',
                                label: Container(
                                    padding: EdgeInsets.all(8.0),
                                    alignment: Alignment.center,
                                    child: Text('MsgName'))),
                            GridColumn(
                                width: columnWidths["Direction"]!,
                                columnName: 'Direction',
                                label: Container(
                                    padding: EdgeInsets.all(8.0),
                                    alignment: Alignment.center,
                                    child: Text(
                                      'Direction',
                                      overflow: TextOverflow.ellipsis,
                                    ))),
                            GridColumn(
                                columnName: 'Signals',
                                visible: false,
                                label: Container(
                                    padding: EdgeInsets.all(8.0),
                                    alignment: Alignment.center,
                                    child: Text('Signals'))),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
                Expanded(flex: 3, child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [Expanded(child: SingleChildScrollView(child: _signalsWidget))],))
              ],
            );
          } else {
            return const Text("loading...........");
          }
        });
  }
}

class FrameDataSource extends DataGridSource {
  DataTable SignalsWidget(List<Signal> signals) {
    return DataTable(
        columns: const <DataColumn>[
          DataColumn(
            label: Expanded(
              child: Text(
                'Signal Name',
                style: TextStyle(fontStyle: FontStyle.italic),
              ),
            ),
          ),
          DataColumn(
            label: Expanded(
              child: Text(
                'Value',
                style: TextStyle(fontStyle: FontStyle.italic),
              ),
            ),
          ),
        ],
        rows: List<DataRow>.generate(
            signals.length,
            (int index) => DataRow(cells: <DataCell>[
                  DataCell(Text('${signals[index].sigName}')),
                  DataCell(Text('${signals[index].sigValue}'))
                ])));
  }

  /// Creates the employee data source class with required details.
  FrameDataSource(List<FrameItem> frames) {
    _frames = frames
        .map<DataGridRow>((e) => DataGridRow(cells: [
              DataGridCell<Widget>(columnName: 'MsgId', value: Text("${e.msgId}")),
              DataGridCell<Widget>(columnName: 'MsgName', value: Text("${e.msgName}")),
              DataGridCell<Widget>(columnName: 'Direction', value: Text("${e.direction}")),
              DataGridCell<Widget>(
                  columnName: 'Signals', value: SignalsWidget(e.signals)),
            ]))
        .toList();
  }

  List<DataGridRow> _frames = [];

  @override
  List<DataGridRow> get rows => _frames;

  @override
  DataGridRowAdapter buildRow(DataGridRow row) {
    return DataGridRowAdapter(
        cells: row.getCells().map<Widget>((e) {
      return Container(
        alignment: Alignment.topLeft,
        padding: const EdgeInsets.all(8.0),
        child: e.value as Widget,
      );
    }).toList());
  }
}


class SignalsWidget extends StatefulWidget {
  SignalsWidget(List<Signal> signals, {super.key}) {
    _signals = signals;
  }

  late List<Signal> _signals;

  updateSignals(List<Signal> signals) {
    _signals = signals;
    _state.updateSignals();
  }

  late _SignalsWidgetState _state;
  @override
  State<SignalsWidget> createState() {
     _state = _SignalsWidgetState();
     return _state;
  }
}

class _SignalsWidgetState extends State<SignalsWidget> {
  updateSignals() {
    setState(() {
    });
  }

  @override
  Widget build(BuildContext context) {
    return DataTable(
        columns: const <DataColumn>[
          DataColumn(
            label: Expanded(
              child: Text(
                'Signal Name',
                style: TextStyle(fontStyle: FontStyle.italic),
              ),
            ),
          ),
          DataColumn(
            label: Expanded(
              child: Text(
                'Value',
                style: TextStyle(fontStyle: FontStyle.italic),
              ),
            ),
          ),
        ],
        rows: List<DataRow>.generate(
            widget._signals.length,
                (int index) => DataRow(cells: <DataCell>[
              DataCell(Text('${widget._signals[index].sigName}')),
              DataCell(Text('${widget._signals[index].sigValue}'))
            ])));
  }
}
