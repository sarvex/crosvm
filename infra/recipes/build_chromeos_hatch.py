# Copyright 2022 The ChromiumOS Authors
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

from recipe_engine.post_process import Filter

DEPS = [
    "crosvm",
    "depot_tools/depot_tools",
    "recipe_engine/buildbucket",
    "recipe_engine/context",
    "recipe_engine/properties",
    "recipe_engine/step",
]


def RunSteps(api):
    with api.crosvm.cros_container_build_context():
        gitilies = api.buildbucket.build.input.gitiles_commit
        upstream_url = "https://chromium.googlesource.com/crosvm/crosvm"
        revision = gitilies.id or "upstream/main"

        api.crosvm.step_in_container(
            "Sync repo",
            [
                "repo",
                "sync",
                "-j8",
            ],
            cros=True,
        )

        api.crosvm.step_in_container(
            "Add crosvm upstream remote",
            ["git", "remote", "add", "upstream", upstream_url],
            cros=True,
        )

        # Ignore errors from unshallow as repo sync sometimes resulted in full git history
        api.crosvm.step_in_container(
            "Unshallow crosvm as needed",
            [
                "bash",
                "-c",
                "for i in $(seq 1 5);do if [[ $(git rev-parse --is-shallow-repository) == 'true' ]]; then git fetch cros --unshallow; else break; fi; done",
            ],
            cros=True,
        )

        api.crosvm.step_in_container("Print current git log", ["git", "log"], cros=True)

        api.crosvm.step_in_container(
            "Fetch upstream crosvm", ["git", "fetch", "upstream"], cros=True
        )

        # Apply unmerged commit from upstream to crOS tree
        api.crosvm.step_in_container(
            "Cherry-pick from upstream revision",
            ["git", "cherry-pick", f"..{revision}"],
            cros=True,
        )

        api.crosvm.step_in_container(
            "cros-workon-hatch crosvm",
            ["cros_sdk", "cros-workon-hatch", "start", "crosvm"],
            cros=True,
        )

        api.crosvm.step_in_container(
            "Build crosvm",
            [
                "cros_sdk",
                "build_packages",
                "--board=hatch",
                "crosvm",
            ],
            cros=True,
        )


def GenTests(api):
    filter_steps = Filter("Build crosvm")
    yield (
        api.test(
            "build_chromeos_hatch",
            api.buildbucket.ci_build(project="crosvm/crosvm"),
        )
        + api.post_process(filter_steps)
    )
