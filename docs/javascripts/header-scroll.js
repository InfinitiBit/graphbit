/**
 * GraphBit Documentation - Header Scroll Behavior
 * Implements smooth scroll behavior for the two-row header
 */

(function() {
  'use strict';

  // Configuration
  const SCROLL_THRESHOLD = 100; // Pixels to scroll before hiding logo header
  const SCROLL_DEBOUNCE = 10; // Debounce scroll events (ms)

  let lastScrollTop = 0;
  let isScrolling = false;
  let scrollTimer = null;

  // Get header element
  const header = document.querySelector('.md-header');
  if (!header) return;

  /**
   * Handle scroll events with debouncing
   */
  function handleScroll() {
    if (scrollTimer) {
      clearTimeout(scrollTimer);
    }

    scrollTimer = setTimeout(() => {
      const currentScrollTop = window.pageYOffset || document.documentElement.scrollTop;
      
      // Determine scroll direction and position
      const isScrollingDown = currentScrollTop > lastScrollTop;
      const hasScrolledPastThreshold = currentScrollTop > SCROLL_THRESHOLD;

      // Add or remove scrolled class based on scroll position and direction
      if (hasScrolledPastThreshold && isScrollingDown) {
        header.classList.add('md-header--scrolled');
      } else if (currentScrollTop <= SCROLL_THRESHOLD) {
        header.classList.remove('md-header--scrolled');
      }

      lastScrollTop = currentScrollTop;
    }, SCROLL_DEBOUNCE);
  }

  /**
   * Initialize scroll behavior
   */
  function initScrollBehavior() {
    // Add scroll event listener
    window.addEventListener('scroll', handleScroll, { passive: true });

    // Handle initial scroll position
    handleScroll();

    // Handle page navigation (for SPA-like behavior in Material)
    document.addEventListener('DOMContentLoaded', handleScroll);
    
    // Handle instant navigation if enabled
    if (typeof app !== 'undefined' && app.location$) {
      app.location$.subscribe(handleScroll);
    }
  }

  /**
   * Cleanup function
   */
  function cleanup() {
    if (scrollTimer) {
      clearTimeout(scrollTimer);
    }
    window.removeEventListener('scroll', handleScroll);
  }

  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initScrollBehavior);
  } else {
    initScrollBehavior();
  }

  // Cleanup on page unload
  window.addEventListener('beforeunload', cleanup);

  // Handle Material's instant navigation
  if (typeof document$ !== 'undefined') {
    document$.subscribe(() => {
      // Re-initialize after navigation
      setTimeout(initScrollBehavior, 100);
    });
  }

})();
