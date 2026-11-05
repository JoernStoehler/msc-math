#!/usr/bin/env python3
"""Render the adjacent compact Markdown checklist as a single A4 page."""
import argparse
from html import escape
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import Flowable, Paragraph, SimpleDocTemplate, Spacer


class CheckRow(Flowable):
    def __init__(self, text, style, width):
        super().__init__()
        self.paragraph = Paragraph(escape(text), style)
        self.width = width
        _, height = self.paragraph.wrap(width - 13, 900)
        self.height = height + 4

    def draw(self):
        self.canv.setStrokeColor(colors.HexColor('#53616b'))
        self.canv.setLineWidth(0.6)
        self.canv.rect(0, self.height - 10, 6.5, 6.5)
        self.paragraph.drawOn(self.canv, 13, 4)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--source', type=Path, default=Path(__file__).with_name('MEETING-CHECKLIST.md'))
    ap.add_argument('--output', type=Path, default=Path(__file__).with_name('MEETING-CHECKLIST.pdf'))
    args = ap.parse_args()
    fonts = Path('/usr/share/fonts/truetype/dejavu')
    pdfmetrics.registerFont(TTFont('DV', str(fonts / 'DejaVuSans.ttf')))
    pdfmetrics.registerFont(TTFont('DV-Bold', str(fonts / 'DejaVuSans-Bold.ttf')))
    width = A4[0] - 34 * mm
    base = ParagraphStyle('base', fontName='DV', fontSize=9, leading=12, alignment=TA_LEFT)
    title = ParagraphStyle('title', parent=base, fontName='DV-Bold', fontSize=18, leading=22, textColor=colors.HexColor('#153b4a'), spaceAfter=7)
    section = ParagraphStyle('section', parent=base, fontName='DV-Bold', fontSize=10.6, leading=14, spaceBefore=8, spaceAfter=5, textColor=colors.HexColor('#153b4a'))
    footer = ParagraphStyle('footer', parent=base, fontSize=7, leading=10)
    doc = SimpleDocTemplate(str(args.output), pagesize=A4, rightMargin=17 * mm, leftMargin=17 * mm, topMargin=15 * mm, bottomMargin=14 * mm, title='Mit Kai: Anmeldung und Prüffassung', author='Jörn Stöhler', pageCompression=1, invariant=1)
    flow = []
    at_footer = False
    for line in args.source.read_text().splitlines():
        if not line:
            continue
        if line.startswith('# '):
            flow.append(Paragraph(escape(line[2:]), title))
        elif line.startswith('## '):
            flow.append(Paragraph(escape(line[3:]), section))
        elif line.startswith('- '):
            flow.append(CheckRow(line[2:], base, width))
        else:
            if line.startswith('Stand:'):
                at_footer = True
                flow.append(Spacer(1, 8))
            flow.append(Paragraph(escape(line), footer if at_footer else base))
            if not at_footer:
                flow.append(Spacer(1, 4))
    doc.build(flow)


if __name__ == '__main__':
    main()
