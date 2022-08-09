import 'dart:convert';
import 'dart:math';

import 'package:desktop_multi_window/desktop_multi_window.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:flutter_hbb/desktop/pages/remote_page.dart';
import 'package:flutter_hbb/desktop/widgets/tabbar_widget.dart';
import 'package:flutter_hbb/utils/multi_window_manager.dart';
import 'package:get/get.dart';
import 'package:window_manager/window_manager.dart';

import '../../models/model.dart';

class ConnectionTabPage extends StatefulWidget {
  final Map<String, dynamic> params;

  const ConnectionTabPage({Key? key, required this.params}) : super(key: key);

  @override
  State<ConnectionTabPage> createState() => _ConnectionTabPageState(params);
}

class _ConnectionTabPageState extends State<ConnectionTabPage>
    with TickerProviderStateMixin {
  // refactor List<int> when using multi-tab
  // this singleton is only for test
  var connectionIds = RxList<String>.empty(growable: true);
  var initialIndex = 0;
  late Rx<TabController> tabController;
  static final Rx<int> _selected = 0.obs;

  var connectionMap = RxList<Widget>.empty(growable: true);

  _ConnectionTabPageState(Map<String, dynamic> params) {
    if (params['id'] != null) {
      connectionIds.add(params['id']);
    }
  }

  @override
  void initState() {
    super.initState();
    tabController =
        TabController(length: connectionIds.length, vsync: this).obs;
    rustDeskWinManager.setMethodHandler((call, fromWindowId) async {
      print(
          "call ${call.method} with args ${call.arguments} from window ${fromWindowId}");
      // for simplify, just replace connectionId
      if (call.method == "new_remote_desktop") {
        window_on_top();
        final args = jsonDecode(call.arguments);
        final id = args['id'];
        final indexOf = connectionIds.indexOf(id);
        if (indexOf >= 0) {
          initialIndex = indexOf;
          tabController.value.animateTo(initialIndex, duration: Duration.zero);
        } else {
          connectionIds.add(id);
          initialIndex = connectionIds.length - 1;
          tabController.value = TabController(
              length: connectionIds.length,
              vsync: this,
              initialIndex: initialIndex);
        }
        _selected.value = initialIndex;
      } else if (call.method == "onDestroy") {
        print("executing onDestroy hook, closing ${connectionIds}");
        connectionIds.forEach((id) {
          final tag = '${id}';
          ffi(tag).close().then((_) {
            Get.delete<FFI>(tag: tag);
          });
        });
        Get.back();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          Obx(() => DesktopTabBar(
                controller: tabController,
                tabs: connectionIds.toList(),
                onTabClose: onRemoveId,
                tabIcon: Icons.desktop_windows_sharp,
                selected: _selected,
              )),
          Expanded(
              child: Obx(() => TabBarView(
                  controller: tabController.value,
                  children: connectionIds
                      .map((e) => RemotePage(
                            key: ValueKey(e),
                            id: e,
                            tabBarHeight: kDesktopRemoteTabBarHeight,
                          )) //RemotePage(key: ValueKey(e), id: e))
                      .toList()))),
        ],
      ),
    );
  }

  void onRemoveId(String id) {
    final indexOf = connectionIds.indexOf(id);
    if (indexOf == -1) {
      return;
    }
    connectionIds.removeAt(indexOf);
    initialIndex = max(0, initialIndex - 1);
    tabController.value = TabController(
        length: connectionIds.length, vsync: this, initialIndex: initialIndex);
    if (connectionIds.length == 0) {
      WindowController.fromWindowId(windowId()).close();
    }
  }

  int windowId() {
    return widget.params["windowId"];
  }
}
