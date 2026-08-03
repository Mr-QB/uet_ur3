#!/usr/bin/env bash

# Source this file in every terminal used by the UET UR3 simulation.  ROS 2
# participants can discover each other only when these discovery settings
# match before the process starts.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    echo "Use: source ${BASH_SOURCE[0]}" >&2
    exit 1
fi

export ROS_DOMAIN_ID="${UET_ROS_DOMAIN_ID:-10}"
export ROS_LOCALHOST_ONLY="${UET_ROS_LOCALHOST_ONLY:-0}"
export RMW_IMPLEMENTATION="${UET_RMW_IMPLEMENTATION:-rmw_fastrtps_cpp}"

# The user's global shell may point Fast DDS at a discovery server on another
# Tailscale machine.  Local simulation must not depend on that machine being
# online, so restore Fast DDS peer discovery for this workspace.
unset ROS_DISCOVERY_SERVER
unset CYCLONEDDS_URI

source /opt/ros/humble/setup.bash

_uet_workspace_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
if [[ -f "${_uet_workspace_root}/install/setup.bash" ]]; then
    source "${_uet_workspace_root}/install/setup.bash"
else
    echo "Warning: ${_uet_workspace_root}/install/setup.bash is missing; build the workspace first." >&2
fi

echo "UET ROS environment: ROS_DOMAIN_ID=${ROS_DOMAIN_ID}, ROS_LOCALHOST_ONLY=${ROS_LOCALHOST_ONLY}, RMW_IMPLEMENTATION=${RMW_IMPLEMENTATION}, discovery=local"
unset _uet_workspace_root
