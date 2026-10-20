# -*- mode: python ; coding: utf-8 -*-

from pathlib import Path

project = Path(SPECPATH).parent.parent
locales = project / "src" / "info_desktop" / "locales"

a = Analysis(
    [str(project / "main.py")],
    pathex=[str(project / "src")],
    binaries=[],
    datas=[(str(locales), "info_desktop/locales")],
    hiddenimports=[],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
)

pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name="INFO-Desktop",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=False,
)

coll = COLLECT(
    exe,
    a.binaries,
    a.datas,
    strip=False,
    upx=True,
    upx_exclude=[],
    name="INFO-Desktop",
)

app = BUNDLE(
    coll,
    name="INFO Desktop.app",
    icon=None,
    bundle_identifier="com.romanzavada.info-desktop",
)
