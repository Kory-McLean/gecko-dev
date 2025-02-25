# ***** BEGIN LICENSE BLOCK *****
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this file,
# You can obtain one at http://mozilla.org/MPL/2.0/.
# ***** END LICENSE BLOCK *****
import os

tooltool_url = 'http://taskcluster/tooltool.mozilla-releng.net/'
if os.environ.get('TASKCLUSTER_ROOT_URL', 'https://taskcluster.net') != 'https://taskcluster.net':
    # Pre-point tooltool at staging cluster so we can run in parallel with legacy cluster
    tooltool_url = 'http://taskcluster/tooltool.staging.mozilla-releng.net/'

config = {
    "options": [
        "--prefs-root=%(test_path)s/prefs",
        "--processes=1",
        "--config=%(test_path)s/wptrunner.ini",
        "--ca-cert-path=%(test_path)s/tests/tools/certs/cacert.pem",
        "--host-key-path=%(test_path)s/tests/tools/certs/web-platform.test.key",
        "--host-cert-path=%(test_path)s/tests/tools/certs/web-platform.test.pem",
        "--certutil-binary=%(xre_path)s/certutil",
        "--product=firefox_android",
    ],
    "avds_dir": "/builds/worker/workspace/build/.android",
    "binary_path": "/tmp",
    "geckodriver": "%(abs_test_bin_dir)s/geckodriver",
    "hostutils_manifest_path": "testing/config/tooltool-manifests/linux64/hostutils.manifest",
    "log_tbpl_level": "info",
    "log_raw_level": "info",
    "minidump_stackwalk_path": "linux64-minidump_stackwalk",
    "per_test_category": "web-platform",
    "tooltool_cache": os.environ.get("TOOLTOOL_CACHE"),
    "tooltool_servers": [tooltool_url],
}
