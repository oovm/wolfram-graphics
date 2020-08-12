v = Table[15{Cos[t], Sin[t]}, {t, 0, 4Pi, 4Pi / 5}];
Graphics[GraphicsComplex[v, {Point[{1, 2, 3, 4, 5, 6}], Green, Line[{1, 2, 3, 4, 5, 6}]}]]