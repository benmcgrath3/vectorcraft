# Upstream sync — 2026-10-12

Original project: `storytold/vectorcraft`, from `2ac3ba2b` through `fecea539`. 138 commits, including merge commits.

Changes include:
- Memory: smaller document objects, shared decoded-image storage, bounded undo and renderer caches; thumbnails render off the UI thread.
- Files: streaming open/save, background open, safer recovery and save failures, editable export metadata, SVG data attributes and export-range checks.
- Imports: bounded EPS path/dash/shading/pattern processing, spot inks and quadratic curves; larger Image Trace inputs handled safely; closed DXF spline flags.
- Editing: fewer no-op Undo steps; safer hidden/locked and isolated selections; repeat-transform options; snap/anchor, stroke, curvature and artboard-tool fixes.
- Graphs: data worksheet, marker designs, label formatting, pie options, filled-line overlap, shadows and legend positioning.
- Text and UI: proportional kerning, full font names, Hungarian UI and expanded Czech translation, plug-in numeric precision, film grain and eyedropper cursor.
- Maintenance: CI action updates and Linux package/runtime fixes.

Local workflows retained: persistent artboard attachment, dimension editing, protected Backspace deletion, external-text paste, selection color replacement, and drag-only new text.

## Validation

All seven `cargo xtask ci` steps passed. The merged macOS app verified selection-color replacement and Undo, attached artwork outside artboard bounds, native save/reopen retaining attachment, and click suppression versus dragged text creation. Native screenshot captured.

## Incoming commits

- 0a538ef4 Docs: ROADMAP 2026-10-12 adds #1245, #1336, #1337 and #1341
- be9d05b2 EPS: the PostScript reader reads many tiling patterns, spot inks and shadings in linear time
- d63c57d1 #1244: Hungarian rows for the new messages in the catalog's own voice (mentsd, jelentsd)
- 6fcdb421 #1244: the status bar stops saying a file is opening once it has opened
- e2f0aed1 #1244: test that a native save failing partway leaves the file as it was
- ecf361c9 #1244: reserve at most 64 MB for a file's entry before its data is read
- ecb4be19 #1244: a crash between a recovery copy's document and its details keeps every image
- 023c8e3e #1244: streaming open and save, per-section limits, recovery that keeps images apart, background open
- 5beac4f1 #1244: smaller objects: Node 352 -> 152 bytes, NodeKind 200 -> 80, and a cheaper undo estimate
- c4214be9 #1244: render caches bounded by bytes, evicting the least recently used instead of clearing
- 454f1737 #1244: perf --deep measures four embedded photos drawn by three renderers
- 91398830 #1244: navigator and layer thumbnails render off the UI thread and wait for drags to end
- bbbafdef #1244: undo history bounded by bytes, without lookup tables or repeated selections in its steps
- fe8a6983 #1244: one shared, byte-budgeted store for decoded images; render caches that sweep and stay bounded
- 76ca890b Path: Simplify and Add Anchor Points record no undo step when they change no path
- 8b6bfe6e Transparency: opacity mask options that change nothing record no undo step
- 17cff377 Docs: ROADMAP 2026-10-12 adds #1325, #1327 and #1329–#1332
- ae27cf6c EPS: loops under a long dash array no longer hang the import
- 9cbab138 Paint: fill, stroke, gradient and Appearance panel commands that change nothing record no undo step
- d0bb09c1 EPS: huge arcs, fine dashes and rectangle arrays in a loop no longer hang the import
- 4a27d897 EPS: shading functions in shared arrays no longer hang the import
- 4412a4ea Plug-ins: share the whole-number field test harness
- 31ae1eba Plug-ins: whole-number fields keep values past 2^53 exactly as typed
- d3be7800 Select: Select All skips hidden isolated groups, and menu commands stay within what it selects
- 15de47b8 Docs: merge duplicated table rows and record the rest of 2026-10-12
- 851edd53 fix(cad): set the closed bit on DXF SPLINE entities so closed curves import into CAM (#1299)
- a24269eb Type: Change Case, Smart Punctuation and other commands that change nothing record no undo step
- c28a9b5d ci: upgrade actions to Node 24 to fix deprecation warnings
- ae5fa87d Image Trace: an oversized image decodes within the allocation its 192 megapixel limit needs (512 MB on the web)
- e1fe2e38 Image Trace: an image over 64 megapixels is traced scaled down
- db8b9bb0 EPS: closepath no longer grows the path past its limit
- f8f40e79 Artboards: the Artboard tool starts on the active artboard
- 48c52787 i18n (hu): rows for the strings added since the branch (graph designs and data, center align, Find Linked Files, Film Grain, Select Segments)
- b9716366 i18n: add Hungarian (hu) UI translation
- e3dec2e8 M14.i18n: translate Czech labels at UI render sites
- 714f1271 M2: reject malformed explicit layer targets (#785)
- 981f6644 fix(M11): close setup before artboard editing
- f602dc3d fix(M11): claim Escape in the Artboard tool
- 5bcd642b Links: Find Linked Files tests build paths with the platform separator
- 7f4f2600 Links: the copy test builds the document folder's paths with the platform separator
- b3f832cb Links: Find in Folder… looks for missing linked files in a folder and its subfolders
- fd2ab03c Layers: Send to Current Layer refuses a hidden or locked current layer
- ac8d7ebc Transform: Transform Again repeats the options of the transform it repeats
- 9ba45990 Part of #394: new and imported documents use the Gridline every and Subdivisions preferences
- 8b839dd5 Snap to Point: drag a selection by the anchor pressed near
- 5cbb7989 Pen + Alt converts anchors of paths in a selected group
- ba4a8707 fix: Simplify後の選択を更新して点数を表示する #1272
- f683f0ff Film Grain dialog: the sliders use the shared grey rail
- 810af8d0 G2: add Artistic Film Grain live effect
- 1475a079 fix(packaging): RPMの依存を共有ライブラリ能力に修正 (#1275)
- d0569b7f perf: テクスチャ効果の距離判定を軽くする #1257
- 876f8e28 Type: Proportional Metrics bring their kerning — `apkn` with `palt`, `vkrn` and `vapk` with `vpal` (#966)
- 68c3cd56 fix: Curvature Toolの点表示を整理する #1236
- ce38eef8 fix: フォントのフルネームを正しい書体へ解決する #1258
- 4109717a fix: 動かないアンカー整列でUndo履歴を増やさない #1180
- 978aa973 fix: スポイトの通常カーソルを十字表示から修正する #1269
- 2dc37d42 fix: ロック中と隔離の外側の図形から色を取得する #1251
- 4149a09d Direct Selection: a Shift-click on a segment pick's endpoint selects that anchor (#1278)
- 1ac79768 fix(selection): 線分の選択と削除をアンカーから分ける (#1278)
- aa15cb52 Direct Selection: Shift-click deselects one anchor of a path selected whole
- 83622239 Appearance panel: a stroke colour on an unpainted zero-width stroke restores 1 pt too (#1253)
- 2f64b5a8 fix: 線の色追加時にゼロ幅を既定値へ戻す #1253
- 86d45dc6 Docs: ROADMAP progress for 2026-10-12 (community PRs landed overnight)
- 4268b8ed Trace: colour mode keeps close dark tones apart
- 44f3141c MCP: lost replies report uncertainty instead of replaying edits
- 650c5a00 Color params: one scale per CMYK colour, not per component
- e1beafb3 Align: keep the tests and panel docs #1073's branch dropped, and translate the new button
- 365e7f28 IO.1: preserve editable export data and AI artboard options
- f9a6c107 PEN.1: fix endpoint picking and commit handle previews
- 21603e6c TEST.1: correct font shaping and embedding oracles
- 7a49c49a SVG.1: preserve data attributes and validate export ranges
- 110bb50e Graph Marker: a None entry goes back to the default marker
- a672bd70 feat(ui): add horizontal and vertical center align button
- 27141b87 Graph Data: a worksheet window
- e4c03b5c Graphs keep their label formatting when drawn again
- 4de410c7 Graph Type: widths past 100%, First Row and First Column in Front, Draw Filled Lines
- fe33dec0 Graph Type: Add Drop Shadow and Add Legend Across Top
- 5e4a282f Graph Data: Transpose and Switch x/y; `|` line breaks in labels; pie Legend, Position and Sort
- 62aceb20 Review: Graph Design dialog as Illustrator's (preview, New Design, Rename, Delete Design, Paste Design, Select Unused)
- 39798722 Graph designs and marker designs: Object › Graph › Design… and Marker…
