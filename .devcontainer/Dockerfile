# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2025 Cyril Jacquet

FROM fedora:42

# Install Qt6, development tools, and X11 support
RUN dnf update -y && \
    # Install core development tools first
    dnf install -y \
        gcc \
        gcc-c++ \
        cmake \
        extra-cmake-modules \
        ninja-build \
        git \
        pkg-config \
        make \
        gdb \
        valgrind \
        clang \
        clang-tools-extra \
        vim \
        nano \
        wget \
        curl \
        unzip \
        tree \
        htop \
        sudo \
        which

    # Install Qt6 packages (may have different availability)
RUN  dnf install -y  \
        qt6-qtbase-devel \
        qt6-qttools-devel \
        qt6-qtdeclarative-devel \
        qt6-qtsvg-devel

RUN  dnf install -y  \
        qcoro-qt6-devel

    # Install X11 packages for GUI testing and xcb-cursor (CRITICAL for Qt 6.5+)
RUN  dnf install -y \
        xorg-x11-server-Xvfb \
        xorg-x11-xauth \
        dbus-x11 \
        mesa-dri-drivers \
        mesa-libGL-devel \
        mesa-libGLU-devel \
        xcb-util-cursor-devel \
        libxcb-devel \
        xcb-util-devel \
    && \
    # Try alternative X11 package names if the above fail
    (dnf install -y xauth 2>/dev/null || echo "xauth not available") && \
    (dnf install -y xvfb 2>/dev/null || echo "xvfb not available") && \
    # Try alternative cursor library names
    (dnf install -y xcb-cursor0 2>/dev/null || echo "xcb-cursor0 not available") && \
    (dnf install -y libxcb-cursor0 2>/dev/null || echo "libxcb-cursor0 not available") && \
    dnf clean all


# Set Qt environment variables
ENV QT_SELECT=6
ENV QT_QPA_PLATFORM=xcb

CMD ["/bin/bash"]
