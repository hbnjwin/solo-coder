package com.linkyoyo.tobacco.repository;

import com.linkyoyo.tobacco.entity.StandardNormativeReference;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;

/**
 * 标准规范性引用文件明细表 Repository
 */
@Repository
public interface StandardNormativeReferenceRepository extends JpaRepository<StandardNormativeReference, Integer> {
    
    /**
     * 根据标准文档ID查询规范性引用文件
     * @param standardDocId 标准文档ID
     * @return 规范性引用文件列表
     */
    List<StandardNormativeReference> findByStandardDocId(Integer standardDocId);
    
    /**
     * 根据标准文档ID删除规范性引用文件
     * @param standardDocId 标准文档ID
     */
    void deleteByStandardDocId(Integer standardDocId);
}
