import pytest
from factors_py import factors

@pytest.mark.parametrize("n,result", [(7, [1, 7]), (9, [1, 3, 9]), (12, [1, 2, 3, 4, 6, 12])])
def test_factors(n, result):
    f = factors(n)
    assert f.factors == result