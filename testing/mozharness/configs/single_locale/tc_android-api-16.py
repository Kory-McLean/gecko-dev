import os

tooltool_url = 'http://taskcluster/tooltool.mozilla-releng.net/'
if os.environ.get('TASKCLUSTER_ROOT_URL', 'https://taskcluster.net') != 'https://taskcluster.net':
    # Pre-point tooltool at staging cluster so we can run in parallel with legacy cluster
    tooltool_url = 'http://taskcluster/tooltool.staging.mozilla-releng.net/'

config = {
    "l10n_dir": "l10n-central",
    "locales_dir": "mobile/android/locales",
    "ignore_locales": ["en-US"],

    "objdir": "obj-firefox",
    "repack_env": {
        "MOZ_OBJDIR": "obj-firefox",
    },

    'vcs_share_base': "/builds/hg-shared",

    "mozconfig": "src/mobile/android/config/mozconfigs/android-api-16/l10n-nightly",
    "tooltool_config": {
        "manifest": "mobile/android/config/tooltool-manifests/android/releng.manifest",
        "output_dir": "%(abs_work_dir)s/src",
    },
    "tooltool_servers": [tooltool_url],

    "upload_env": {
        'UPLOAD_PATH': '/builds/worker/artifacts/',
    },
    'secret_files': [
        {'filename': '/builds/gls-gapi.data',
         'secret_name': 'project/releng/gecko/build/level-%(scm-level)s/gls-gapi.data',
         'min_scm_level': 1},
        {'filename': '/builds/sb-gapi.data',
         'secret_name': 'project/releng/gecko/build/level-%(scm-level)s/sb-gapi.data',
         'min_scm_level': 1},
        {'filename': '/builds/mozilla-fennec-geoloc-api.key',
         'secret_name': 'project/releng/gecko/build/level-%(scm-level)s/mozilla-fennec-geoloc-api.key',
         'min_scm_level': 2, 'default': 'try-build-has-no-secrets'},
    ],
}
