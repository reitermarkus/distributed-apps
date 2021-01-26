\NeedsTeXFormat{LaTeX2e}
 \ProvidesClass{DPSbachelor}[2008/03/03 DPS bachelor theses document class]
\newif\if@mainmatter\@mainmattertrue
\LoadClass[a4paper,11pt,abstracton,twoside]{scrreprt}
\RequirePackage{fancyhdr}
\RequirePackage[english,ngerman]{babel}
\RequirePackage{a4}
\RequirePackage{picins}
\RequirePackage{graphicx}
\RequirePackage{amsmath}
\RequirePackage{amssymb}
\RequirePackage{amsthm}
%\RequirePackage[%
% bookmarks,%
% plainpages=false,%
% pdfpagelabels,%
% colorlinks=false,%
% pdfborder={0 0 0},%
%]{hyperref}
\DeclareOption{english}{\AtBeginDocument{\selectlanguage{english}}}
\DeclareOption{german}{\AtBeginDocument{\selectlanguage{ngerman}}}
\ExecuteOptions{english}
\ProcessOptions\relax
\AtBeginDocument{%
 \global\newbox\absbox%
 \renewcommand\abstractname{%
  \iflanguage{english}{Abstract}{Zusammenfassung}%
 }
 \newcommand\ackname{%
  \iflanguage{english}{Acknowledgments}{Danksagung}%
 }
 \definetoday
}
\newcommand\definetoday{%
 \renewcommand*\today{%
  \iflanguage{english}{%
   \number\day\space \ifcase\month\or%
    January\or%
    February\or%
    March\or%
    April\or%
    May\or%
    June\or%
    July\or%
    August\or%
    September\or%
    October\or%
    November\or%
    December\or%
   \fi\space \number\year
  }{%
   \number\day.~\ifcase\month\or%
    J\"anner\or%
    Februar\or%
    M\"arz\or%
    April\or%
    Mai\or%
    Juni\or%
    Juli\or%
    August\or%
    September\or%
    Oktober\or%
    November\or%
    Dezember%
   \fi\space \number\year
  }%
 }
}
\let\@OLDappendix\appendix
\renewcommand\appendix{%
 \backmatter
\phantomsection{\addcontentsline{toc}{chapter}{\bibname}}
 \bibliographystyle{plain}
 \bibliography{\@bibfile}
 \@OLDappendix
}
\theoremstyle{plain}
\newtheorem{corollary}{Corollary}[%
chapter%
]
\newtheorem{lemma}[corollary]{Lemma}
\newtheorem{proposition}[corollary]{Proposition}
\newtheorem{theorem}[corollary]{Theorem}

\theoremstyle{definition}
\newtheorem{definition}[corollary]{Definition}
\newtheorem{example}[corollary]{Example}
\newcommand\@bibfile{biblio}
\newcommand\bibfile[1]{\renewcommand\@bibfile{#1}}
\newcommand*\@mailaddress\@empty
\global\let\@matriculationnumber\@empty
\newcommand*\mailaddress[1]{\gdef\@mailaddress{#1}}
\newcommand*\matriculationnumber[1]{\gdef\@matriculationnumber{#1}}
\newcommand*\@plaintitle\@empty
\newcommand*\@plainauthor\@empty
\newcommand*\plaintitle[1]{\gdef\@plaintitle{#1}}
\newcommand*\plainauthor[1]{\gdef\@plainauthor{#1}}
\newcommand\kindname{%
 \iflanguage{english}{Project Report}{Projekt}%
}
\newcommand\supervisorname{%
 \iflanguage{english}{Supervisor}{Betreuer}%
}

\newcommand\university{%
 \iflanguage{english}{Leopold--Franzens--University \\ Austria, Innsbruck \\}
 {Leopold--Franzens--Universität \\ Innsbruck \\}
}

\newcommand\institute{%
 \iflanguage{english}{Institute of Computer Science\\Distributed and Parallel Systems Group\\ Distributed Applications in the Edge-Cloud Continuum 2020W}
 {Institut für Informatik \\ Forschungsgruppe DPS \\ (Distributed and Parallel Systems) \\}%
}

\newcommand\degreename{Project Report}
\subject{\kindname}
\newcommand{\supervisor}[1]{%
 \def\@supervisor{#1}
 \publishers{\textbf{\supervisorname:} #1}
}
\renewcommand\abstract[1]{%
 \global\setbox\absbox=\hbox{#1}%
}
\global\let\@acknowledgments\@empty
\newcommand\acknowledgments[1]{\def\@acknowledgments{#1}}
\newcommand*\frontmatter{%
 \if@twoside\cleardoublepage\else\clearpage\fi
  \@mainmatterfalse\pagenumbering{roman}%
}
\newcommand*\mainmatter{%
 \if@twoside\cleardoublepage\else\clearpage\fi
  \@mainmattertrue\pagenumbering{arabic}%
}
\newcommand*\backmatter{%
\if@openright\cleardoublepage\else\clearpage\fi
  \@mainmatterfalse%
}
\renewcommand*\maketitle[1][-1]{{%
 \frontmatter
 \thispagestyle{empty}
 \let\footnotesize\small
 \let\footnoterule\relax
 \let\footnote\thanks
 \renewcommand*\thefootnote{\@fnsymbol\c@footnote}%
 \let\@oldmakefnmark\@makefnmark
 \renewcommand*{\@makefnmark}{\rlap\@oldmakefnmark}


 \parpic[l]{
  \includegraphics[width=5.7cm]{unilogo_title}
 }

 \begin{flushright}
  {\normalfont \sffamily
   {\LARGE \university}
   \vspace{\stretch{1}}
   {\vspace{1cm}\textbf {\large \institute \ }}
   \vspace{\stretch{2}}
   {\LARGE \\ \textbf{\@title} \\ \ }
   \vspace{\stretch{1}}
   {\textbf {\large \\ \kindname \\ \ }}
   {\large \\ \@publishers \\ \ }
   \vspace{\stretch{1}}
   {\bfseries
    {\large \\ \@author
    \ifx\@matriculationnumber\@empty{}\else{~(\@matriculationnumber)}\fi \\ \
    }
   }
   {\large \\ \href{mailto:\@mailaddress}{\@mailaddress} \\ \ }
   \vspace{\stretch{1}}
   {\large \\ Innsbruck \\ \today{} \\ \ }
  }
 \end{flushright}




 \@thanks
 \global\let\@thanks\@empty
 \vfill\null
 \vskip 3em
 \@thanks
 %\vfill\null
 \setcounter{footnote}{0}%
 \global\let\thanks\relax
 \global\let\maketitle\relax
 \global\let\@thanks\@empty
 \global\let\@author\@empty
 \global\let\@date\@empty
 \global\let\@title\@empty
 \global\let\@extratitle\@empty
 \global\let\@titlehead\@empty
 \global\let\@subject\@empty
 \global\let\@publishers\@empty
 \global\let\@uppertitleback\@empty
 \global\let\@lowertitleback\@empty
 \global\let\@dedication\@empty
 \global\let\author\relax
 \global\let\title\relax
 \global\let\extratitle\relax
 \global\let\titlehead\relax
 \global\let\subject\relax
 \global\let\publishers\relax
 \global\let\uppertitleback\relax
 \global\let\lowertitleback\relax
 \global\let\dedication\relax
 \global\let\date\relax
 \global\let\and\relax
}}
\let\@OLDtableofcontents\tableofcontents
\renewcommand\tableofcontents{%
 \pagestyle{empty}
 \cleardoublepage
 \begin{center}
 \normalfont\sectfont\nobreak\abstractname
 \@endparpenalty\@M
 \end{center}
 \unhbox\absbox
 \par\vfil\null
 \ifx\@acknowledgments\@empty{}\else
  \cleardoublepage
  \chapter*{\ackname}
  \@acknowledgments
  \cleardoublepage
 \fi
 \@OLDtableofcontents
\cleardoublepage
 \mainmatter
 \pagestyle{fancy}
 \fancyhead{}
 \fancyfoot{}
 \renewcommand\chaptermark[1]{\markboth{\thechapter\ ##1}{}}
 \renewcommand\sectionmark[1]{\markright{\thesection\ ##1}}
 \renewcommand\headrulewidth{0.5pt}
 \fancyhead[LE]{\leftmark}
 \fancyhead[RO]{\rightmark}
 \fancyfoot[LE,RO]{\thepage}
}
\endinput

