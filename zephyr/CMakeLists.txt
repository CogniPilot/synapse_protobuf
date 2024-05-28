# SPDX-License-Identifier: Apache-2.0
if (CONFIG_SYNAPSE_PROTOBUF)

list(APPEND CMAKE_MODULE_PATH ${ZEPHYR_BASE}/modules/nanopb)
include(nanopb)

set(SYNAPSE_PROTOBUF_DIR ${ZEPHYR_CURRENT_MODULE_DIR})
set(PROTO_DIR ${SYNAPSE_PROTOBUF_DIR}/proto)
 
file(GLOB OPTION_FILES "${PROTO_DIR}/*.option")

message(STATUS "files:${PROTO_FILES}")
   
set(NANOPB_IMPORT_DIRS "${PROTO_DIR}")
set(NANOPB_OPTIONS "-I${CMAKE_SOURCE_DIR}")
message(STATUS "srcs:${PROTO_SOURCES}")

zephyr_library_named(synapse_protobuf)

# sorted in dependency order
set(PROTO_FILES
    ${PROTO_DIR}/actuators.proto
    ${PROTO_DIR}/altimeter.proto
    ${PROTO_DIR}/battery_state.proto
    ${PROTO_DIR}/bezier_curve.proto
    ${PROTO_DIR}/bezier_trajectory.proto
    ${PROTO_DIR}/can.proto
    ${PROTO_DIR}/time.proto
    ${PROTO_DIR}/header.proto
    ${PROTO_DIR}/vector3.proto
    ${PROTO_DIR}/quaternion.proto
    ${PROTO_DIR}/imu.proto
    ${PROTO_DIR}/joy.proto
    ${PROTO_DIR}/led.proto
    ${PROTO_DIR}/led_array.proto
    ${PROTO_DIR}/magnetic_field.proto
    ${PROTO_DIR}/nav_sat_fix.proto
    ${PROTO_DIR}/nav_sat_status.proto
    ${PROTO_DIR}/point.proto
    ${PROTO_DIR}/pose.proto
    ${PROTO_DIR}/pose_with_covariance.proto
    ${PROTO_DIR}/safety.proto
    ${PROTO_DIR}/sim_clock.proto
    ${PROTO_DIR}/status.proto
    ${PROTO_DIR}/twist.proto
    ${PROTO_DIR}/twist_with_covariance.proto
    ${PROTO_DIR}/wheel_odometry.proto
    ${PROTO_DIR}/odometry.proto
    )

foreach(FILE ${PROTO_FILES})
  zephyr_nanopb_sources(synapse_protobuf ${FILE})
endforeach()

# we need to be able to include generated header files
zephyr_library_include_directories(
  ${CMAKE_CURRENT_BINARY_DIR}/../
)

zephyr_include_directories(
  ${CMAKE_CURRENT_BINARY_DIR}/../
)

# necessary for module to calll nanopb_generate
add_dependencies(app synapse_protobuf)

endif()
