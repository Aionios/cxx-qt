// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QPointF>
#include <QtCore/QSizeF>
#include <QtCore/QString>
#include <QtGui/QColor>

#include "cxx-qt-gen/include/lib.rs.h"

bool
test_constructed_qstring(const QString& s);

void
assign_to_qstring(QString& s, const QString& v);

bool
test_constructed_qcolor(const QColor& c, ColorTest test);
