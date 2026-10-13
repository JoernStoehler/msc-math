import importlib.util
from pathlib import Path
import pytest
spec=importlib.util.spec_from_file_location('trace_time',Path(__file__).resolve().parents[1]/'scripts/trace_time.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
def trace(spans):return {'schema':'measured-spans/v1','start':0,'end':12,'spans':spans}
def span(id,cat,a,b):return {'id':id,'category':cat,'start':a,'end':b,'timing_basis':'captured boundaries'}
def test_parallel_workers_not_summed_as_wall_time():
 out=module.analyze(trace([span('a','tool',0,10),span('b','tool',0,10)]))
 assert out['summed_span_seconds']==20 and out['category_union_seconds']['tool']==10
 assert out['unobserved_seconds']==2

def test_wait_and_tool_overlap_retained_not_forced_into_partition():
 out=module.analyze(trace([span('a','tool',0,8),span('b','user_wait',6,10)]))
 assert out['cross_category_overlap_seconds']==2 and out['observed_union_seconds']==10
 assert out['unobserved_seconds']==2

def test_missing_execution_boundaries_are_not_inferred_from_transcript():
 with pytest.raises(KeyError):module.analyze(trace([{'id':'event-only','category':'unknown','start':2,'timing_basis':'transcript receipt'}]))
 with pytest.raises(ValueError):module.analyze(trace([span('reversed','tool',8,4)]))
